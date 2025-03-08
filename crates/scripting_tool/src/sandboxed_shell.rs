/// Models will commonly generate POSIX shell one-liner commands which
/// they run via io.popen() in Lua. Instead of giving those shell command
/// strings to the operating system - which is a security risk, and
/// which can eaisly fail on Windows, since Windows doesn't do POSIX - we
/// parse the shell command ourselves and translate it into a sequence of
/// commands in our normal sandbox. Essentially, this is an extremely
/// minimalstic shell which Lua popen() commands can execute in.
use mlua::{Error, Result};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ShellCmd {
    command: String,
    args: Vec<String>,
    stdout_redirect: Option<String>,
    stderr_redirect: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    /// The `|` shell operator
    Pipe,
    /// The `&&` shell operator
    And,
    /// The `;` shell operator
    Semicolon,
}

impl ShellCmd {
    /// Parse a shell string, which we assume the model will generate in POSIX format.
    /// Note that since we are turning this into our own representation, this should
    /// work seamlessly on Windows too, even though Windows has a different shell syntax.
    ///
    /// If there are multiple commands piped into one another, this returns them all.
    pub fn parse_shell_str(string: impl AsRef<str>) -> Result<(Self, Vec<(Operator, Self)>)> {
        let string = string.as_ref();
        // For now, we don't support any of these shell features. We can add support them
        // in the future, though.
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

        // Split the string into individual commands based on connectors
        let mut first_cmd = None;
        let mut other_cmds = Vec::new();
        let mut current_cmd = ShellCmd::default();
        let mut pending_op = Operator::Semicolon;

        macro_rules! finish_cmd {
            ($new_pending_op:expr) => {
                if current_cmd.command.is_empty() {
                    if current_cmd.stdout_redirect.is_some()
                        || current_cmd.stderr_redirect.is_some()
                    {
                        // This means we finished a command that was all redirects
                        // and no command, e.g. something like:
                        // `; > foo.txt 2> bar.txt ;`
                        return Err(Error::RuntimeError(
                            "Invalid operator sequence in this shell".to_string(),
                        ));
                    }
                } else {
                    let cmd = std::mem::take(&mut current_cmd);

                    if first_cmd.is_none() {
                        first_cmd = Some(cmd);
                    } else {
                        other_cmds.push((pending_op, cmd));
                    }

                    pending_op = $new_pending_op;
                }
            };
        }

        let mut lexer = shlex::Shlex::new(string);

        // Process tokens to split commands
        while let Some(token) = lexer.next() {
            match dbg!(token.as_str()) {
                "|" => {
                    finish_cmd!(Operator::Pipe);
                }
                ";" => {
                    finish_cmd!(Operator::Semicolon);
                }
                "&&" => {
                    finish_cmd!(Operator::And);
                }
                "&" => {
                    return Err(Error::RuntimeError(
                        "Background processes (&) are not supported in this shell".to_string(),
                    ));
                }
                ">" | "1>" => {
                    // stdout redirection
                    let target = lexer.next().ok_or_else(|| {
                        Error::RuntimeError("Missing redirection target in shell".to_string())
                    })?;
                    current_cmd.stdout_redirect = Some(target);
                }
                "2>" => {
                    // stderr redirection
                    let target = lexer.next().ok_or_else(|| {
                        Error::RuntimeError("Missing redirection target in shell".to_string())
                    })?;
                    current_cmd.stderr_redirect = Some(target);
                }
                "&>" | ">&" => {
                    // both stdout and stderr redirection
                    let target = lexer.next().ok_or_else(|| {
                        Error::RuntimeError("Missing redirection target in shell".to_string())
                    })?;
                    current_cmd.stdout_redirect = Some(target.clone());
                    current_cmd.stderr_redirect = Some(target);
                }
                _ => {
                    let original_token_len = token.len();
                    let mut token = token;

                    // The lexer keeps trailing semicolons in tokens.
                    while token.ends_with(';') {
                        token.pop();
                    }

                    let had_semicolon = token.len() != original_token_len;

                    if current_cmd.command.is_empty() {
                        current_cmd.command = token;
                    } else {
                        current_cmd.args.push(token);
                    }

                    if had_semicolon {
                        finish_cmd!(Operator::Semicolon);
                    }
                }
            }
        }

        // We ran out of tokens; finish the last command we were working on.
        finish_cmd!(Operator::Semicolon);

        // This silences a warning about pending_op being assigned in the
        // finish_cmd! macro and then never read again, without silencing
        // it in all macro invocations.
        drop(pending_op);

        if let Some(cmd) = first_cmd {
            Ok((cmd, other_cmds))
        } else {
            Err(Error::RuntimeError(
                "Missing command to run in shell".to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_command() {
        // Basic command with no args or operators
        let cmd = "ls";
        let expected = (
            ShellCmd {
                command: "ls".to_string(),
                args: vec![],
                stdout_redirect: None,
                stderr_redirect: None,
            },
            vec![],
        );

        assert_eq!(
            expected,
            ShellCmd::parse_shell_str(cmd).expect("parsing failed for {cmd:?}")
        );
    }

    #[test]
    fn test_command_with_args() {
        // Command with arguments
        let cmd = "ls -la /home";
        let expected = (
            ShellCmd {
                command: "ls".to_string(),
                args: vec!["-la".to_string(), "/home".to_string()],
                stdout_redirect: None,
                stderr_redirect: None,
            },
            vec![],
        );

        assert_eq!(
            expected,
            ShellCmd::parse_shell_str(cmd).expect("parsing failed for {cmd:?}")
        );
    }

    #[test]
    fn test_simple_pipe() {
        // Test pipe operator
        let cmd = "ls -l | grep txt";
        let expected = (
            ShellCmd {
                command: "ls".to_string(),
                args: vec!["-l".to_string()],
                stdout_redirect: None,
                stderr_redirect: None,
            },
            vec![(
                Operator::Pipe,
                ShellCmd {
                    command: "grep".to_string(),
                    args: vec!["txt".to_string()],
                    stdout_redirect: None,
                    stderr_redirect: None,
                },
            )],
        );

        assert_eq!(
            expected,
            ShellCmd::parse_shell_str(cmd).expect("parsing failed for {cmd:?}")
        );
    }

    #[test]
    fn test_simple_and() {
        // Test && operator
        let cmd = "mkdir test && cd test";
        let expected = (
            ShellCmd {
                command: "mkdir".to_string(),
                args: vec!["test".to_string()],
                stdout_redirect: None,
                stderr_redirect: None,
            },
            vec![(
                Operator::And,
                ShellCmd {
                    command: "cd".to_string(),
                    args: vec!["test".to_string()],
                    stdout_redirect: None,
                    stderr_redirect: None,
                },
            )],
        );

        assert_eq!(
            expected,
            ShellCmd::parse_shell_str(cmd).expect("parsing failed for {cmd:?}")
        );
    }

    #[test]
    fn test_simple_semicolon() {
        // Test ; operator
        let cmd = "echo hello ; echo world";
        let expected = (
            ShellCmd {
                command: "echo".to_string(),
                args: vec!["hello".to_string()],
                stdout_redirect: None,
                stderr_redirect: None,
            },
            vec![(
                Operator::Semicolon,
                ShellCmd {
                    command: "echo".to_string(),
                    args: vec!["world".to_string()],
                    stdout_redirect: None,
                    stderr_redirect: None,
                },
            )],
        );

        assert_eq!(
            expected,
            ShellCmd::parse_shell_str(cmd).expect("parsing failed for {cmd:?}")
        );
    }

    #[test]
    fn test_stdout_redirection() {
        // Test stdout redirection
        let cmd = "echo hello > output.txt";
        let expected = (
            ShellCmd {
                command: "echo".to_string(),
                args: vec!["hello".to_string()],
                stdout_redirect: Some("output.txt".to_string()),
                stderr_redirect: None,
            },
            vec![],
        );

        assert_eq!(
            expected,
            ShellCmd::parse_shell_str(cmd).expect("parsing failed for {cmd:?}")
        );
    }

    #[test]
    fn test_stderr_redirection() {
        // Test stderr redirection
        let cmd = "find / -name test 2> errors.log";
        let expected = (
            ShellCmd {
                command: "find".to_string(),
                args: vec!["/".to_string(), "-name".to_string(), "test".to_string()],
                stdout_redirect: None,
                stderr_redirect: Some("errors.log".to_string()),
            },
            vec![],
        );

        assert_eq!(
            expected,
            ShellCmd::parse_shell_str(cmd).expect("parsing failed for {cmd:?}")
        );
    }

    #[test]
    fn test_both_redirections() {
        // Test both stdout and stderr redirection
        let cmd = "make &> build.log";
        let expected = (
            ShellCmd {
                command: "make".to_string(),
                args: vec![],
                stdout_redirect: Some("build.log".to_string()),
                stderr_redirect: Some("build.log".to_string()),
            },
            vec![],
        );

        assert_eq!(
            expected,
            ShellCmd::parse_shell_str(cmd).expect("parsing failed for {cmd:?}")
        );

        // Test alternative syntax
        let cmd = "make >& build.log";
        let expected = (
            ShellCmd {
                command: "make".to_string(),
                args: vec![],
                stdout_redirect: Some("build.log".to_string()),
                stderr_redirect: Some("build.log".to_string()),
            },
            vec![],
        );

        assert_eq!(
            expected,
            ShellCmd::parse_shell_str(cmd).expect("parsing failed for {cmd:?}")
        );
    }

    #[test]
    fn test_multiple_operators() {
        // Test multiple operators in a single command
        let cmd =
            "find . -name \"*.rs\" | grep impl && echo \"Found implementations\" ; echo \"Done\"";
        let expected = (
            ShellCmd {
                command: "find".to_string(),
                args: vec![".".to_string(), "-name".to_string(), "*.rs".to_string()],
                stdout_redirect: None,
                stderr_redirect: None,
            },
            vec![
                (
                    Operator::Pipe,
                    ShellCmd {
                        command: "grep".to_string(),
                        args: vec!["impl".to_string()],
                        stdout_redirect: None,
                        stderr_redirect: None,
                    },
                ),
                (
                    Operator::And,
                    ShellCmd {
                        command: "echo".to_string(),
                        args: vec!["Found implementations".to_string()],
                        stdout_redirect: None,
                        stderr_redirect: None,
                    },
                ),
                (
                    Operator::Semicolon,
                    ShellCmd {
                        command: "echo".to_string(),
                        args: vec!["Done".to_string()],
                        stdout_redirect: None,
                        stderr_redirect: None,
                    },
                ),
            ],
        );

        assert_eq!(
            expected,
            ShellCmd::parse_shell_str(cmd).expect("parsing failed for {cmd:?}")
        );
    }

    #[test]
    fn test_pipe_with_redirections() {
        // Test pipe with redirections
        let cmd = "cat file.txt | grep error > results.txt 2> errors.log";
        let expected = (
            ShellCmd {
                command: "cat".to_string(),
                args: vec!["file.txt".to_string()],
                stdout_redirect: None,
                stderr_redirect: None,
            },
            vec![(
                Operator::Pipe,
                ShellCmd {
                    command: "grep".to_string(),
                    args: vec!["error".to_string()],
                    stdout_redirect: Some("results.txt".to_string()),
                    stderr_redirect: Some("errors.log".to_string()),
                },
            )],
        );

        assert_eq!(
            expected,
            ShellCmd::parse_shell_str(cmd).expect("parsing failed for {cmd:?}")
        );
    }

    #[test]
    fn test_quoted_arguments() {
        // Test quoted arguments
        let cmd = "echo \"hello world\" | grep \"o w\"";
        let expected = (
            ShellCmd {
                command: "echo".to_string(),
                args: vec!["hello world".to_string()],
                stdout_redirect: None,
                stderr_redirect: None,
            },
            vec![(
                Operator::Pipe,
                ShellCmd {
                    command: "grep".to_string(),
                    args: vec!["o w".to_string()],
                    stdout_redirect: None,
                    stderr_redirect: None,
                },
            )],
        );

        assert_eq!(
            expected,
            ShellCmd::parse_shell_str(cmd).expect("parsing failed for {cmd:?}")
        );
    }

    #[test]
    fn test_unsupported_features() {
        // Test unsupported shell features
        let result = ShellCmd::parse_shell_str("echo $HOME");
        assert!(result.is_err());

        let result = ShellCmd::parse_shell_str("echo `date`");
        assert!(result.is_err());

        let result = ShellCmd::parse_shell_str("echo $(date)");
        assert!(result.is_err());

        let result = ShellCmd::parse_shell_str("for i in {1..5}; do echo $i; done");
        assert!(result.is_err());
    }

    #[test]
    fn test_complex_command() {
        let cmd = "find /path/to/dir -type f -name \"*.txt\" -exec grep \"pattern with spaces\";";
        let expected = (
            ShellCmd {
                command: "find".to_string(),
                args: vec![
                    "/path/to/dir".to_string(),
                    "-type".to_string(),
                    "f".to_string(),
                    "-name".to_string(),
                    "*.txt".to_string(),
                    "-exec".to_string(),
                    "grep".to_string(),
                    "pattern with spaces".to_string(),
                ],
                stdout_redirect: None,
                stderr_redirect: None,
            },
            vec![],
        );

        assert_eq!(
            expected,
            ShellCmd::parse_shell_str(cmd).expect("parsing failed for {cmd:?}")
        );
    }

    #[test]
    fn test_empty_command() {
        // Test empty command
        let result = ShellCmd::parse_shell_str("");
        assert!(result.is_err());
    }

    #[test]
    fn test_missing_redirection_target() {
        // Test missing redirection target
        let result = ShellCmd::parse_shell_str("echo hello >");
        assert!(result.is_err());

        let result = ShellCmd::parse_shell_str("ls 2>");
        assert!(result.is_err());
    }

    #[test]
    fn test_ampersand_as_argument() {
        // Test & as a background operator is not allowed
        let result = ShellCmd::parse_shell_str("grep & file.txt");
        assert!(result.is_err());

        // Verify the error message mentions background processes
        if let Err(Error::RuntimeError(msg)) = ShellCmd::parse_shell_str("grep & file.txt") {
            assert!(msg.contains("Background processes"));
        } else {
            panic!("Expected RuntimeError about background processes");
        }
    }
}
