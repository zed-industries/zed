/// Models will commonly generate POSIX shell one-liner commands which
/// they run via io.popen() in Lua. Instead of giving those shell command
/// strings to the operating system - which is a security risk, and
/// which can eaisly fail on Windows, since Windows doesn't do POSIX - we
/// parse the shell command ourselves and translate it into a sequence of
/// commands in our normal sandbox. Essentially, this is an extremely
/// minimalstic shell which Lua popen() commands can execute in.
use mlua::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ShellCmd {
    command: String,
    args: Vec<String>,
    stdout_redirect: Option<String>,
    stderr_redirect: Option<String>,
}

impl ShellCmd {
    /// Parse a shell string, which we assume the model will generate in POSIX format.
    /// Note that since we are turning this into our own representation, this should
    /// work seamlessly on Windows too, even though Windows has a different shell syntax.
    ///
    /// If there are multiple commands piped into one another, this returns them all.
    pub fn parse_shell_str(string: &str) -> Result<Vec<Self>> {
        // For now, we don't support any of these shell features. We can add support them
        // in the future, though.
        if string.contains('$')
            || string.contains('`')
            || string.contains('(')
            || string.contains(')')
            || string.contains(';')
            || string.contains('&')
            || string.contains('{')
            || string.contains('}')
        {
            return Err(Error::RuntimeError(
                "Complex shell features (pipes, subshells, variables, etc.) are not available in this shell."
                    .to_string(),
            ));
        }

        // Use shlex to split the command line into tokens
        let tokens = shlex::split(string)
            .ok_or_else(|| Error::RuntimeError("Failed to parse shell command".to_string()))?;

        // The first token is the command
        let mut tokens_iter = tokens.into_iter();
        let Some(command) = tokens_iter.next() else {
            return Err(Error::RuntimeError("Missing popen command".to_string()));
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
                        Error::RuntimeError("Missing redirection target".to_string())
                    })?;
                    stdout_redirect = Some(target);
                }
                "2>" => {
                    // stderr redirection
                    let target = tokens_iter.next().ok_or_else(|| {
                        Error::RuntimeError("Missing redirection target".to_string())
                    })?;
                    stderr_redirect = Some(target);
                }
                "&>" | ">&" => {
                    // both stdout and stderr redirection
                    let target = tokens_iter.next().ok_or_else(|| {
                        Error::RuntimeError("Missing redirection target".to_string())
                    })?;
                    stdout_redirect = Some(target.clone());
                    stderr_redirect = Some(target);
                }
                _ if token.starts_with(">")
                    || token.starts_with("2>")
                    || token.starts_with("&>") =>
                {
                    // Handle cases like ">file" without a space
                    return Err(Error::RuntimeError(
                        "Redirections must have a space between operator and target".to_string(),
                    ));
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
