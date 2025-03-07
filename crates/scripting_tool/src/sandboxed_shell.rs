/// Models will commonly generate POSIX shell one-liner commands which
/// they run via io.popen() in Lua. Instead of giving those shell command
/// strings to the operating system - which is a security risk, and
/// which can eaisly fail on Windows, since Windows doesn't do POSIX - we
/// parse the shell command ourselves and translate it into a sequence of
/// commands in our normal sandbox. Essentially, this is an extremely
/// minimalstic shell which Lua popen() commands can execute in.
use mlua::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShellCmd {
    command: String,
    args: Vec<String>,
    stdout_redirect: Option<String>,
    stderr_redirect: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
        let mut command_strings = Vec::new();
        let mut connector_types = Vec::new();
        let mut current_cmd = String::new();
        let mut current_args = Vec::new();

        // Create a lexer for the shell string
        let mut lexer = shlex::Shlex::new(string);

        // Process tokens to split commands
        while let Some(token) = lexer.next() {
            match token.as_str() {
                "|" => {
                    if !current_cmd.is_empty() {
                        let joined_cmd = if current_args.is_empty() {
                            current_cmd.clone()
                        } else {
                            format!("{} {}", current_cmd, current_args.join(" "))
                        };
                        command_strings.push(joined_cmd);
                        connector_types.push(Operator::Pipe);
                        current_cmd = String::new();
                        current_args = Vec::new();
                    }
                }
                "&" => {
                    // Check if next token is also "&" (for "&&" operator)
                    if let Some(next_token) = lexer.next() {
                        if next_token == "&" {
                            if !current_cmd.is_empty() {
                                let joined_cmd = if current_args.is_empty() {
                                    current_cmd.clone()
                                } else {
                                    format!("{} {}", current_cmd, current_args.join(" "))
                                };
                                command_strings.push(joined_cmd);
                                connector_types.push(Operator::And);
                                current_cmd = String::new();
                                current_args = Vec::new();
                            }
                        } else {
                            // It was a single &, not &&
                            if current_cmd.is_empty() {
                                current_cmd = token;
                            } else {
                                current_args.push(token);
                            }
                            // Don't forget to process the next_token we peeked
                            if current_cmd.is_empty() {
                                current_cmd = next_token;
                            } else {
                                current_args.push(next_token);
                            }
                        }
                    } else {
                        // Just a single & at the end
                        if current_cmd.is_empty() {
                            current_cmd = token;
                        } else {
                            current_args.push(token);
                        }
                    }
                }
                ";" => {
                    if !current_cmd.is_empty() {
                        let joined_cmd = if current_args.is_empty() {
                            current_cmd.clone()
                        } else {
                            format!("{} {}", current_cmd, current_args.join(" "))
                        };
                        command_strings.push(joined_cmd);
                        connector_types.push(Operator::Semicolon);
                        current_cmd = String::new();
                        current_args = Vec::new();
                    }
                }
                _ => {
                    if current_cmd.is_empty() {
                        current_cmd = token;
                    } else {
                        current_args.push(token);
                    }
                }
            }
        }

        // Add the last command if there is one
        if !current_cmd.is_empty() {
            let joined_cmd = if current_args.is_empty() {
                current_cmd.clone()
            } else {
                format!("{} {}", current_cmd, current_args.join(" "))
            };
            command_strings.push(joined_cmd);
        }

        if command_strings.is_empty() {
            return Err(Error::RuntimeError(
                "Missing command to run in shell".to_string(),
            ));
        }

        // Parse the first command to return separately
        let first_cmd = Self::parse_single_command(&command_strings[0])?;

        // Parse the remaining commands
        let mut connected_cmds = Vec::new();
        for i in 1..command_strings.len() {
            let cmd = Self::parse_single_command(&command_strings[i])?;
            connected_cmds.push((connector_types[i - 1].clone(), cmd));
        }

        Ok((first_cmd, connected_cmds))
    }

    // Helper method to parse a single command
    fn parse_single_command(cmd_str: &str) -> Result<Self> {
        // Use shlex to split the command line into tokens
        let tokens = shlex::split(cmd_str).ok_or_else(|| {
            Error::RuntimeError(format!("Failed to parse shell command: {}", cmd_str))
        })?;

        // The first token is the command
        let mut tokens_iter = tokens.into_iter();
        let Some(command) = tokens_iter.next() else {
            return Err(Error::RuntimeError(
                "Missing command to run in shell".to_string(),
            ));
        };

        let mut args = Vec::new();
        let mut stdout_redirect = None;
        let mut stderr_redirect = None;

        // Process the remaining tokens
        while let Some(token) = tokens_iter.next() {
            match token.as_str() {
                ">" | "1>" => {
                    // stdout redirection
                    let target = tokens_iter.next().ok_or_else(|| {
                        Error::RuntimeError("Missing redirection target in shell".to_string())
                    })?;
                    stdout_redirect = Some(target);
                }
                "2>" => {
                    // stderr redirection
                    let target = tokens_iter.next().ok_or_else(|| {
                        Error::RuntimeError("Missing redirection target in shell".to_string())
                    })?;
                    stderr_redirect = Some(target);
                }
                "&>" | ">&" => {
                    // both stdout and stderr redirection
                    let target = tokens_iter.next().ok_or_else(|| {
                        Error::RuntimeError("Missing redirection target in shell".to_string())
                    })?;
                    stdout_redirect = Some(target.clone());
                    stderr_redirect = Some(target);
                }
                _ => {
                    // Regular argument
                    args.push(token);
                }
            }
        }

        Ok(ShellCmd {
            command,
            args,
            stdout_redirect,
            stderr_redirect,
        })
    }
}
