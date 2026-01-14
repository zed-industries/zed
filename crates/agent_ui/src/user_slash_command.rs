use anyhow::{Result, anyhow};
use collections::HashMap;
use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// An error that occurred while loading a command file.
#[derive(Debug, Clone)]
pub struct CommandLoadError {
    /// The path to the file that failed to load
    pub path: PathBuf,
    /// A description of the error
    pub message: String,
}

impl std::fmt::Display for CommandLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to load {}: {}",
            self.path.display(),
            self.message
        )
    }
}

/// Result of loading commands, including any errors encountered.
#[derive(Debug, Default)]
pub struct CommandLoadResult {
    /// Successfully loaded commands
    pub commands: Vec<UserSlashCommand>,
    /// Errors encountered while loading commands
    pub errors: Vec<CommandLoadError>,
}

/// The scope of a user-defined slash command.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandScope {
    /// Project-specific command from .zed/commands/
    Project,
    /// User-wide command from config_dir()/commands/
    User,
}

/// A user-defined slash command loaded from a markdown file.
#[derive(Debug, Clone, PartialEq)]
pub struct UserSlashCommand {
    /// The command name for invocation.
    /// For commands in subdirectories, this is prefixed: "namespace:name" (e.g., "frontend:component")
    /// For commands in the root, this is just the filename without .md extension.
    pub name: Arc<str>,
    /// The template content from the file
    pub template: Arc<str>,
    /// The namespace (subdirectory path, if any), used for description display
    pub namespace: Option<Arc<str>>,
    /// The full path to the command file
    pub path: PathBuf,
    /// Whether this is a project or user command
    pub scope: CommandScope,
}

impl UserSlashCommand {
    /// Returns a description string for display in completions.
    /// Format: "(project)", "(project:namespace)", "(user)", or "(user:namespace)"
    pub fn description(&self) -> String {
        let scope_name = match self.scope {
            CommandScope::Project => "project",
            CommandScope::User => "user",
        };
        match &self.namespace {
            Some(ns) => format!("({}:{})", scope_name, ns),
            None => format!("({})", scope_name),
        }
    }

    /// Returns true if this command has any placeholders ($1, $2, etc. or $ARGUMENTS)
    pub fn requires_arguments(&self) -> bool {
        has_placeholders(&self.template)
    }
}

/// Parsed user command from input text
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedUserCommand<'a> {
    pub name: &'a str,
    pub raw_arguments: &'a str,
}

/// Returns the path to the user commands directory.
pub fn user_commands_dir() -> PathBuf {
    paths::config_dir().join("commands")
}

/// Returns the path to the project commands directory for a given worktree root.
pub fn project_commands_dir(worktree_root: &Path) -> PathBuf {
    worktree_root.join(".zed").join("commands")
}

/// Loads all user slash commands from the user commands directory.
/// Commands are markdown files in `config_dir()/commands/`.
/// Subdirectories create namespaces.
pub fn load_user_commands() -> CommandLoadResult {
    let commands_path = user_commands_dir();
    load_commands_from_path(&commands_path, CommandScope::User)
}

/// Loads all project slash commands from a worktree root.
/// Commands are markdown files in `.zed/commands/`.
/// Subdirectories create namespaces.
pub fn load_project_commands(worktree_root: &Path) -> CommandLoadResult {
    let commands_path = project_commands_dir(worktree_root);
    load_commands_from_path(&commands_path, CommandScope::Project)
}

/// Loads all commands (both project and user) for given worktree roots.
/// If multiple commands have the same name, an error is reported for the ambiguity.
/// Returns both successfully loaded commands and any errors encountered.
pub fn load_all_commands(worktree_roots: &[PathBuf]) -> CommandLoadResult {
    let mut result = CommandLoadResult::default();
    let mut seen_commands: std::collections::HashMap<String, PathBuf> =
        std::collections::HashMap::new();

    // Load project commands first
    for root in worktree_roots {
        let project_result = load_project_commands(root);
        result.errors.extend(project_result.errors);
        for cmd in project_result.commands {
            if let Some(existing_path) = seen_commands.get(&*cmd.name) {
                result.errors.push(CommandLoadError {
                    path: cmd.path.clone(),
                    message: format!(
                        "Command '{}' is ambiguous: also defined at {}",
                        cmd.name,
                        existing_path.display()
                    ),
                });
            } else {
                seen_commands.insert(cmd.name.to_string(), cmd.path.clone());
                result.commands.push(cmd);
            }
        }
    }

    // Load user commands
    let user_result = load_user_commands();
    result.errors.extend(user_result.errors);
    for cmd in user_result.commands {
        if let Some(existing_path) = seen_commands.get(&*cmd.name) {
            result.errors.push(CommandLoadError {
                path: cmd.path.clone(),
                message: format!(
                    "Command '{}' is ambiguous: also defined at {}",
                    cmd.name,
                    existing_path.display()
                ),
            });
        } else {
            seen_commands.insert(cmd.name.to_string(), cmd.path.clone());
            result.commands.push(cmd);
        }
    }

    result
}

fn load_commands_from_path(commands_path: &Path, scope: CommandScope) -> CommandLoadResult {
    let mut result = CommandLoadResult::default();

    if !commands_path.exists() {
        return result;
    }

    load_commands_from_dir(commands_path, commands_path, scope, &mut result);
    result
}

fn load_commands_from_dir(
    base_path: &Path,
    current_path: &Path,
    scope: CommandScope,
    result: &mut CommandLoadResult,
) {
    let entries = match std::fs::read_dir(current_path) {
        Ok(entries) => entries,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return,
        Err(e) => {
            result.errors.push(CommandLoadError {
                path: current_path.to_path_buf(),
                message: format!("Failed to read directory: {}", e),
            });
            return;
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                result.errors.push(CommandLoadError {
                    path: current_path.to_path_buf(),
                    message: format!("Failed to read directory entry: {}", e),
                });
                continue;
            }
        };
        let path = entry.path();

        if path.is_dir() {
            load_commands_from_dir(base_path, &path, scope, result);
        } else if path.extension().is_some_and(|ext| ext == "md") {
            match load_command_file(base_path, &path, scope) {
                Ok(Some(command)) => result.commands.push(command),
                Ok(None) => {} // Empty file, skip silently
                Err(e) => {
                    result.errors.push(CommandLoadError {
                        path: path.clone(),
                        message: e.to_string(),
                    });
                }
            }
        }
    }
}

fn load_command_file(
    base_path: &Path,
    file_path: &Path,
    scope: CommandScope,
) -> Result<Option<UserSlashCommand>> {
    let base_name = match file_path.file_stem() {
        Some(stem) => stem.to_string_lossy().into_owned(),
        None => return Ok(None),
    };

    let template = std::fs::read_to_string(file_path)?;
    if template.is_empty() {
        return Ok(None);
    }

    let namespace = file_path
        .parent()
        .and_then(|parent| parent.strip_prefix(base_path).ok())
        .filter(|rel| !rel.as_os_str().is_empty())
        .map(|rel| {
            rel.to_string_lossy()
                .replace(std::path::MAIN_SEPARATOR, "/")
        });

    // Build the full command name: "namespace:basename" or just "basename"
    let name = match &namespace {
        Some(ns) => format!("{}:{}", ns.replace('/', ":"), base_name),
        None => base_name,
    };

    Ok(Some(UserSlashCommand {
        name: name.into(),
        template: template.into(),
        namespace: namespace.map(|s| s.into()),
        path: file_path.to_path_buf(),
        scope,
    }))
}

/// Converts a list of UserSlashCommand to a HashMap for quick lookup.
/// The key is the command name.
pub fn commands_to_map(commands: &[UserSlashCommand]) -> HashMap<String, UserSlashCommand> {
    let mut map = HashMap::default();
    for cmd in commands {
        map.insert(cmd.name.to_string(), cmd.clone());
    }
    map
}

/// Parses a line of input to extract a user command invocation.
/// Returns None if the line doesn't start with a slash command.
pub fn try_parse_user_command(line: &str) -> Option<ParsedUserCommand<'_>> {
    let line = line.trim_start();
    if !line.starts_with('/') {
        return None;
    }

    let after_slash = &line[1..];
    let (name, raw_arguments) = if let Some(space_idx) = after_slash.find(char::is_whitespace) {
        let name = &after_slash[..space_idx];
        let rest = &after_slash[space_idx..].trim_start();
        (name, *rest)
    } else {
        (after_slash, "")
    };

    if name.is_empty() {
        return None;
    }

    Some(ParsedUserCommand {
        name,
        raw_arguments,
    })
}

/// Parses command arguments, supporting quoted strings.
/// - Unquoted arguments are space-separated
/// - Quoted arguments can contain spaces: "multi word arg"
/// - Escape sequences: \" for literal quote, \\ for backslash, \n for newline
pub fn parse_arguments(input: &str) -> Result<Vec<Cow<'_, str>>> {
    let mut arguments = Vec::new();
    let mut chars = input.char_indices().peekable();

    while let Some((start_idx, c)) = chars.next() {
        if c.is_whitespace() {
            continue;
        }

        if c == '"' {
            let mut result = String::new();
            let mut closed = false;

            while let Some((_, ch)) = chars.next() {
                if ch == '\\' {
                    if let Some((_, next_ch)) = chars.next() {
                        match next_ch {
                            '"' => result.push('"'),
                            '\\' => result.push('\\'),
                            'n' => result.push('\n'),
                            other => {
                                return Err(anyhow!("Unknown escape sequence: \\{}", other));
                            }
                        }
                    } else {
                        return Err(anyhow!("Unexpected end of input after backslash"));
                    }
                } else if ch == '"' {
                    closed = true;
                    break;
                } else {
                    result.push(ch);
                }
            }

            if !closed {
                return Err(anyhow!("Unclosed quote in command arguments"));
            }

            arguments.push(Cow::Owned(result));
        } else {
            let mut end_idx = start_idx + c.len_utf8();
            while let Some(&(idx, ch)) = chars.peek() {
                if ch.is_whitespace() {
                    break;
                }
                if ch == '"' {
                    return Err(anyhow!("Quote in middle of unquoted argument"));
                }
                end_idx = idx + ch.len_utf8();
                chars.next();
            }

            arguments.push(Cow::Borrowed(&input[start_idx..end_idx]));
        }
    }

    Ok(arguments)
}

/// Checks if a template has any placeholders ($1, $2, etc. or $ARGUMENTS)
pub fn has_placeholders(template: &str) -> bool {
    count_positional_placeholders(template) > 0 || template.contains("$ARGUMENTS")
}

/// Counts the highest positional placeholder number in the template.
/// For example, "$1 and $3" returns 3.
pub fn count_positional_placeholders(template: &str) -> usize {
    let mut max_placeholder = 0;
    let mut chars = template.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            chars.next();
            continue;
        }
        if c == '$' {
            let mut num_str = String::new();
            while let Some(&next_c) = chars.peek() {
                if next_c.is_ascii_digit() {
                    num_str.push(next_c);
                    chars.next();
                } else {
                    break;
                }
            }
            if !num_str.is_empty() {
                if let Ok(n) = num_str.parse::<usize>() {
                    max_placeholder = max_placeholder.max(n);
                }
            }
        }
    }

    max_placeholder
}

/// Validates that arguments match the template's placeholders.
/// Templates can use $ARGUMENTS (all args as one string) or $1, $2, etc. (positional).
pub fn validate_arguments(
    command_name: &str,
    template: &str,
    arguments: &[Cow<'_, str>],
    raw_arguments: &str,
) -> Result<()> {
    if template.is_empty() {
        return Err(anyhow!("Template cannot be empty"));
    }

    let has_arguments_placeholder = template.contains("$ARGUMENTS");
    let positional_count = count_positional_placeholders(template);

    if has_arguments_placeholder {
        // $ARGUMENTS accepts any number of arguments (including zero)
        // But if there are also positional placeholders, validate those
        if positional_count > 0 && arguments.len() < positional_count {
            return Err(anyhow!(
                "The /{} command requires {} positional {}, but only {} {} provided",
                command_name,
                positional_count,
                if positional_count == 1 {
                    "argument"
                } else {
                    "arguments"
                },
                arguments.len(),
                if arguments.len() == 1 { "was" } else { "were" }
            ));
        }
        return Ok(());
    }

    if positional_count == 0 && !arguments.is_empty() {
        return Err(anyhow!(
            "The /{} command accepts no arguments, but {} {} provided",
            command_name,
            arguments.len(),
            if arguments.len() == 1 { "was" } else { "were" }
        ));
    }

    if arguments.len() < positional_count {
        return Err(anyhow!(
            "The /{} command requires {} {}, but only {} {} provided",
            command_name,
            positional_count,
            if positional_count == 1 {
                "argument"
            } else {
                "arguments"
            },
            arguments.len(),
            if arguments.len() == 1 { "was" } else { "were" }
        ));
    }

    if arguments.len() > positional_count {
        return Err(anyhow!(
            "The /{} command accepts {} {}, but {} {} provided",
            command_name,
            positional_count,
            if positional_count == 1 {
                "argument"
            } else {
                "arguments"
            },
            arguments.len(),
            if arguments.len() == 1 { "was" } else { "were" }
        ));
    }

    let _ = raw_arguments; // Used by $ARGUMENTS
    Ok(())
}

/// Expands a template by substituting placeholders with arguments.
/// - $ARGUMENTS is replaced with all arguments as a single string
/// - $1, $2, etc. are replaced with positional arguments
/// - \$ produces literal $, \" produces literal ", \n produces newline
pub fn expand_template(
    template: &str,
    arguments: &[Cow<'_, str>],
    raw_arguments: &str,
) -> Result<String> {
    let mut result = String::with_capacity(template.len());
    let mut chars = template.char_indices().peekable();

    while let Some((_, c)) = chars.next() {
        if c == '\\' {
            if let Some((_, next_c)) = chars.next() {
                match next_c {
                    '$' => result.push('$'),
                    '"' => result.push('"'),
                    '\\' => result.push('\\'),
                    'n' => result.push('\n'),
                    other => {
                        return Err(anyhow!("Unknown escape sequence: \\{}", other));
                    }
                }
            }
        } else if c == '$' {
            // Check for $ARGUMENTS first
            let remaining: String = chars.clone().map(|(_, c)| c).collect();
            if remaining.starts_with("ARGUMENTS") {
                result.push_str(raw_arguments);
                // Skip "ARGUMENTS"
                for _ in 0..9 {
                    chars.next();
                }
            } else {
                // Check for positional placeholder $N
                let mut num_str = String::new();
                while let Some(&(_, next_c)) = chars.peek() {
                    if next_c.is_ascii_digit() {
                        num_str.push(next_c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if !num_str.is_empty() {
                    let n: usize = num_str.parse()?;
                    if n == 0 {
                        return Err(anyhow!(
                            "Placeholder $0 is invalid; placeholders start at $1"
                        ));
                    }
                    if let Some(arg) = arguments.get(n - 1) {
                        result.push_str(arg);
                    } else {
                        return Err(anyhow!("Missing argument for placeholder ${}", n));
                    }
                } else {
                    result.push('$');
                }
            }
        } else {
            result.push(c);
        }
    }

    Ok(result)
}

/// Expands a user slash command, validating arguments and performing substitution.
pub fn expand_user_slash_command(
    command_name: &str,
    template: &str,
    arguments: &[Cow<'_, str>],
    raw_arguments: &str,
) -> Result<String> {
    validate_arguments(command_name, template, arguments, raw_arguments)?;
    expand_template(template, arguments, raw_arguments)
}

/// Attempts to expand a user slash command from input text.
/// Returns Ok(None) if the input is not a user command or the command doesn't exist.
/// Returns Err if the command exists but expansion fails (e.g., missing arguments).
pub fn try_expand_from_commands(
    line: &str,
    commands: &HashMap<String, UserSlashCommand>,
) -> Result<Option<String>> {
    let Some(parsed) = try_parse_user_command(line) else {
        return Ok(None);
    };

    let Some(command) = commands.get(parsed.name) else {
        return Ok(None);
    };

    let arguments = parse_arguments(parsed.raw_arguments)?;
    let expanded = expand_user_slash_command(
        parsed.name,
        &command.template,
        &arguments,
        parsed.raw_arguments,
    )?;
    Ok(Some(expanded))
}

/// Legacy function for compatibility with settings-based commands.
/// Attempts to expand a user slash command from input text using a simple HashMap.
#[cfg(test)]
fn try_expand_user_slash_command(
    line: &str,
    slash_commands: &HashMap<String, String>,
) -> Result<Option<String>> {
    let Some(parsed) = try_parse_user_command(line) else {
        return Ok(None);
    };

    let Some(template) = slash_commands.get(parsed.name) else {
        return Ok(None);
    };

    let arguments = parse_arguments(parsed.raw_arguments)?;
    let expanded =
        expand_user_slash_command(parsed.name, template, &arguments, parsed.raw_arguments)?;
    Ok(Some(expanded))
}

/// Checks if a command name exists in the user commands.
pub fn has_command(name: &str, commands: &HashMap<String, UserSlashCommand>) -> bool {
    commands.contains_key(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_parse_user_command() {
        assert_eq!(
            try_parse_user_command("/review"),
            Some(ParsedUserCommand {
                name: "review",
                raw_arguments: ""
            })
        );

        assert_eq!(
            try_parse_user_command("/review arg1 arg2"),
            Some(ParsedUserCommand {
                name: "review",
                raw_arguments: "arg1 arg2"
            })
        );

        assert_eq!(
            try_parse_user_command("/cmd \"multi word\" simple"),
            Some(ParsedUserCommand {
                name: "cmd",
                raw_arguments: "\"multi word\" simple"
            })
        );

        assert_eq!(try_parse_user_command("not a command"), None);
        assert_eq!(try_parse_user_command(""), None);
        assert_eq!(try_parse_user_command("/"), None);
    }

    #[test]
    fn test_parse_arguments_simple_unquoted() {
        let args = parse_arguments("foo bar").unwrap();
        assert_eq!(args, vec!["foo", "bar"]);
    }

    #[test]
    fn test_parse_arguments_quoted() {
        let args = parse_arguments("\"foo bar\"").unwrap();
        assert_eq!(args, vec!["foo bar"]);
    }

    #[test]
    fn test_parse_arguments_mixed() {
        let args = parse_arguments("\"foo bar\" baz \"qux\"").unwrap();
        assert_eq!(args, vec!["foo bar", "baz", "qux"]);
    }

    #[test]
    fn test_parse_arguments_escaped_quotes() {
        let args = parse_arguments("\"foo \\\"bar\\\" baz\"").unwrap();
        assert_eq!(args, vec!["foo \"bar\" baz"]);
    }

    #[test]
    fn test_parse_arguments_escaped_backslash() {
        let args = parse_arguments("\"foo\\\\bar\"").unwrap();
        assert_eq!(args, vec!["foo\\bar"]);
    }

    #[test]
    fn test_parse_arguments_empty_quoted() {
        let args = parse_arguments("\"\"").unwrap();
        assert_eq!(args, vec![""]);
    }

    #[test]
    fn test_parse_arguments_unclosed_quote_error() {
        let result = parse_arguments("\"foo");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unclosed quote"));
    }

    #[test]
    fn test_parse_arguments_quote_in_middle_error() {
        let result = parse_arguments("foo\"bar");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Quote in middle"));
    }

    #[test]
    fn test_parse_arguments_unknown_escape_error() {
        let result = parse_arguments("\"\\x\"");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unknown escape"));
    }

    #[test]
    fn test_parse_arguments_newline_escape() {
        let args = parse_arguments("\"line1\\nline2\"").unwrap();
        assert_eq!(args, vec!["line1\nline2"]);
    }

    #[test]
    fn test_count_positional_placeholders() {
        assert_eq!(count_positional_placeholders("Hello $1"), 1);
        assert_eq!(count_positional_placeholders("$1 and $2"), 2);
        assert_eq!(count_positional_placeholders("$1 $1"), 1);
        assert_eq!(count_positional_placeholders("$2 then $1"), 2);
        assert_eq!(count_positional_placeholders("no placeholders"), 0);
        assert_eq!(count_positional_placeholders("\\$1 escaped"), 0);
        assert_eq!(count_positional_placeholders("$10 big number"), 10);
    }

    #[test]
    fn test_has_placeholders() {
        assert!(has_placeholders("Hello $1"));
        assert!(has_placeholders("$ARGUMENTS"));
        assert!(has_placeholders("prefix $ARGUMENTS suffix"));
        assert!(!has_placeholders("no placeholders"));
        assert!(!has_placeholders("\\$1 escaped"));
    }

    #[test]
    fn test_expand_template_basic() {
        let args = vec![Cow::Borrowed("world")];
        let result = expand_template("Hello $1", &args, "world").unwrap();
        assert_eq!(result, "Hello world");
    }

    #[test]
    fn test_expand_template_multiple_placeholders() {
        let args = vec![Cow::Borrowed("a"), Cow::Borrowed("b")];
        let result = expand_template("$1 and $2", &args, "a b").unwrap();
        assert_eq!(result, "a and b");
    }

    #[test]
    fn test_expand_template_repeated_placeholder() {
        let args = vec![Cow::Borrowed("x")];
        let result = expand_template("$1 $1", &args, "x").unwrap();
        assert_eq!(result, "x x");
    }

    #[test]
    fn test_expand_template_out_of_order() {
        let args = vec![Cow::Borrowed("a"), Cow::Borrowed("b")];
        let result = expand_template("$2 then $1", &args, "a b").unwrap();
        assert_eq!(result, "b then a");
    }

    #[test]
    fn test_expand_template_newline_escape() {
        let args: Vec<Cow<'_, str>> = vec![];
        let result = expand_template("line1\\nline2", &args, "").unwrap();
        assert_eq!(result, "line1\nline2");
    }

    #[test]
    fn test_expand_template_dollar_escape() {
        let args: Vec<Cow<'_, str>> = vec![];
        let result = expand_template("cost is \\$1", &args, "").unwrap();
        assert_eq!(result, "cost is $1");
    }

    #[test]
    fn test_expand_template_quote_escape() {
        let args: Vec<Cow<'_, str>> = vec![];
        let result = expand_template("say \\\"hi\\\"", &args, "").unwrap();
        assert_eq!(result, "say \"hi\"");
    }

    #[test]
    fn test_expand_template_backslash_escape() {
        let args: Vec<Cow<'_, str>> = vec![];
        let result = expand_template("path\\\\file", &args, "").unwrap();
        assert_eq!(result, "path\\file");
    }

    #[test]
    fn test_expand_template_arguments_placeholder() {
        let args = vec![Cow::Borrowed("foo"), Cow::Borrowed("bar")];
        let result = expand_template("All args: $ARGUMENTS", &args, "foo bar").unwrap();
        assert_eq!(result, "All args: foo bar");
    }

    #[test]
    fn test_expand_template_arguments_with_positional() {
        let args = vec![Cow::Borrowed("first"), Cow::Borrowed("second")];
        let result = expand_template("First: $1, All: $ARGUMENTS", &args, "first second").unwrap();
        assert_eq!(result, "First: first, All: first second");
    }

    #[test]
    fn test_expand_template_arguments_empty() {
        let args: Vec<Cow<'_, str>> = vec![];
        let result = expand_template("Args: $ARGUMENTS", &args, "").unwrap();
        assert_eq!(result, "Args: ");
    }

    #[test]
    fn test_expand_template_arguments_preserves_quotes() {
        let args = vec![Cow::Borrowed("multi word")];
        let result = expand_template("Args: $ARGUMENTS", &args, "\"multi word\"").unwrap();
        assert_eq!(result, "Args: \"multi word\"");
    }

    #[test]
    fn test_validate_arguments_exact_match() {
        let args = vec![Cow::Borrowed("a"), Cow::Borrowed("b")];
        let result = validate_arguments("test", "$1 $2", &args, "a b");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_arguments_missing_args() {
        let args = vec![Cow::Borrowed("a")];
        let result = validate_arguments("foo", "$1 $2", &args, "a");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("/foo"));
        assert!(err.contains("requires 2 arguments"));
        assert!(err.contains("only 1 was provided"));
    }

    #[test]
    fn test_validate_arguments_extra_args() {
        let args = vec![Cow::Borrowed("a"), Cow::Borrowed("b")];
        let result = validate_arguments("foo", "$1", &args, "a b");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("/foo"));
        assert!(err.contains("accepts 1 argument"));
        assert!(err.contains("2 were provided"));
    }

    #[test]
    fn test_validate_arguments_no_placeholders_no_args() {
        let args: Vec<Cow<'_, str>> = vec![];
        let result = validate_arguments("test", "no placeholders here", &args, "");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_arguments_no_placeholders_has_args() {
        let args = vec![Cow::Borrowed("unexpected")];
        let result = validate_arguments("test", "no placeholders here", &args, "unexpected");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("accepts no arguments")
        );
    }

    #[test]
    fn test_validate_arguments_empty_template() {
        let args: Vec<Cow<'_, str>> = vec![];
        let result = validate_arguments("test", "", &args, "");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_validate_arguments_with_arguments_placeholder() {
        // $ARGUMENTS accepts any number of arguments
        let args: Vec<Cow<'_, str>> = vec![];
        let result = validate_arguments("test", "Do: $ARGUMENTS", &args, "");
        assert!(result.is_ok());

        let args = vec![Cow::Borrowed("a"), Cow::Borrowed("b"), Cow::Borrowed("c")];
        let result = validate_arguments("test", "Do: $ARGUMENTS", &args, "a b c");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_arguments_mixed_placeholders() {
        // Both $ARGUMENTS and positional - need at least the positional ones
        let args = vec![Cow::Borrowed("first")];
        let result = validate_arguments("test", "$1 then $ARGUMENTS", &args, "first");
        assert!(result.is_ok());

        let args: Vec<Cow<'_, str>> = vec![];
        let result = validate_arguments("test", "$1 then $ARGUMENTS", &args, "");
        assert!(result.is_err());
    }

    #[test]
    fn test_expand_user_slash_command() {
        let result = expand_user_slash_command(
            "review",
            "Please review: $1",
            &[Cow::Borrowed("security")],
            "security",
        )
        .unwrap();
        assert_eq!(result, "Please review: security");
    }

    #[test]
    fn test_try_expand_user_slash_command() {
        let mut commands = HashMap::default();
        commands.insert("review".to_string(), "Please review: $1".to_string());
        commands.insert("explain".to_string(), "Explain at $1 level: $2".to_string());

        let result = try_expand_user_slash_command("/review security", &commands).unwrap();
        assert_eq!(result, Some("Please review: security".to_string()));

        let result =
            try_expand_user_slash_command("/explain \"beginner\" \"this code\"", &commands)
                .unwrap();
        assert_eq!(
            result,
            Some("Explain at beginner level: this code".to_string())
        );

        let result = try_expand_user_slash_command("/unknown arg", &commands).unwrap();
        assert_eq!(result, None);

        let result = try_expand_user_slash_command("not a command", &commands).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn test_try_expand_user_slash_command_with_missing_args() {
        let mut commands = HashMap::default();
        commands.insert("review".to_string(), "Please review: $1".to_string());

        let result = try_expand_user_slash_command("/review", &commands);
        assert!(result.is_err());
    }

    #[test]
    fn test_try_expand_user_slash_command_with_arguments() {
        let mut commands = HashMap::default();
        commands.insert("search".to_string(), "Search for: $ARGUMENTS".to_string());

        let result = try_expand_user_slash_command("/search foo bar baz", &commands).unwrap();
        assert_eq!(result, Some("Search for: foo bar baz".to_string()));

        let result = try_expand_user_slash_command("/search", &commands).unwrap();
        assert_eq!(result, Some("Search for: ".to_string()));
    }

    #[test]
    fn test_user_slash_command_description() {
        let cmd = UserSlashCommand {
            name: "test".into(),
            template: "test".into(),
            namespace: None,
            path: PathBuf::from("/test.md"),
            scope: CommandScope::User,
        };
        assert_eq!(cmd.description(), "(user)");

        let cmd = UserSlashCommand {
            name: "frontend:test".into(),
            template: "test".into(),
            namespace: Some("frontend".into()),
            path: PathBuf::from("/frontend/test.md"),
            scope: CommandScope::User,
        };
        assert_eq!(cmd.description(), "(user:frontend)");

        let cmd = UserSlashCommand {
            name: "tools:git:test".into(),
            template: "test".into(),
            namespace: Some("tools/git".into()),
            path: PathBuf::from("/tools/git/test.md"),
            scope: CommandScope::User,
        };
        assert_eq!(cmd.description(), "(user:tools/git)");

        let cmd = UserSlashCommand {
            name: "test".into(),
            template: "test".into(),
            namespace: None,
            path: PathBuf::from("/test.md"),
            scope: CommandScope::Project,
        };
        assert_eq!(cmd.description(), "(project)");

        let cmd = UserSlashCommand {
            name: "frontend:test".into(),
            template: "test".into(),
            namespace: Some("frontend".into()),
            path: PathBuf::from("/frontend/test.md"),
            scope: CommandScope::Project,
        };
        assert_eq!(cmd.description(), "(project:frontend)");
    }

    #[test]
    fn test_user_slash_command_requires_arguments() {
        let cmd = UserSlashCommand {
            name: "test".into(),
            template: "No placeholders here".into(),
            namespace: None,
            path: PathBuf::from("/test.md"),
            scope: CommandScope::User,
        };
        assert!(!cmd.requires_arguments());

        let cmd = UserSlashCommand {
            name: "test".into(),
            template: "Hello $1".into(),
            namespace: None,
            path: PathBuf::from("/test.md"),
            scope: CommandScope::User,
        };
        assert!(cmd.requires_arguments());

        let cmd = UserSlashCommand {
            name: "test".into(),
            template: "Do: $ARGUMENTS".into(),
            namespace: None,
            path: PathBuf::from("/test.md"),
            scope: CommandScope::User,
        };
        assert!(cmd.requires_arguments());
    }

    #[test]
    fn test_load_command_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let commands_dir = temp_dir.path();

        // Create a simple command file
        let review_path = commands_dir.join("review.md");
        std::fs::write(&review_path, "Please review this code for: $1").unwrap();

        let result =
            super::load_command_file(commands_dir, &review_path, CommandScope::User).unwrap();
        assert!(result.is_some());
        let cmd = result.unwrap();
        assert_eq!(cmd.name.as_ref(), "review");
        assert_eq!(cmd.template.as_ref(), "Please review this code for: $1");
        assert!(cmd.namespace.is_none());
        assert_eq!(cmd.scope, CommandScope::User);
    }

    #[test]
    fn test_load_command_file_project_scope() {
        let temp_dir = tempfile::tempdir().unwrap();
        let commands_dir = temp_dir.path();

        let review_path = commands_dir.join("review.md");
        std::fs::write(&review_path, "Project review: $1").unwrap();

        let result =
            super::load_command_file(commands_dir, &review_path, CommandScope::Project).unwrap();
        assert!(result.is_some());
        let cmd = result.unwrap();
        assert_eq!(cmd.scope, CommandScope::Project);
        assert_eq!(cmd.description(), "(project)");
    }

    #[test]
    fn test_load_command_file_with_namespace() {
        let temp_dir = tempfile::tempdir().unwrap();
        let commands_dir = temp_dir.path();

        // Create a namespaced command file
        let frontend_dir = commands_dir.join("frontend");
        std::fs::create_dir_all(&frontend_dir).unwrap();
        let component_path = frontend_dir.join("component.md");
        std::fs::write(&component_path, "Create a React component: $1").unwrap();

        let result =
            super::load_command_file(commands_dir, &component_path, CommandScope::User).unwrap();
        assert!(result.is_some());
        let cmd = result.unwrap();
        // Command name should be prefixed with namespace
        assert_eq!(cmd.name.as_ref(), "frontend:component");
        assert_eq!(cmd.template.as_ref(), "Create a React component: $1");
        assert_eq!(cmd.namespace.as_ref().map(|s| s.as_ref()), Some("frontend"));
    }

    #[test]
    fn test_load_command_file_nested_namespace() {
        let temp_dir = tempfile::tempdir().unwrap();
        let commands_dir = temp_dir.path();

        // Create a deeply nested command file
        let nested_dir = commands_dir.join("tools").join("git");
        std::fs::create_dir_all(&nested_dir).unwrap();
        let commit_path = nested_dir.join("commit.md");
        std::fs::write(&commit_path, "Create a git commit: $ARGUMENTS").unwrap();

        let result =
            super::load_command_file(commands_dir, &commit_path, CommandScope::User).unwrap();
        assert!(result.is_some());
        let cmd = result.unwrap();
        // Nested namespace uses : separator
        assert_eq!(cmd.name.as_ref(), "tools:git:commit");
        assert_eq!(
            cmd.namespace.as_ref().map(|s| s.as_ref()),
            Some("tools/git")
        );
    }

    #[test]
    fn test_load_command_file_empty_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let commands_dir = temp_dir.path();

        // Create an empty command file
        let empty_path = commands_dir.join("empty.md");
        std::fs::write(&empty_path, "").unwrap();

        let result =
            super::load_command_file(commands_dir, &empty_path, CommandScope::User).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_load_commands_from_dir() {
        let temp_dir = tempfile::tempdir().unwrap();
        let commands_dir = temp_dir.path();

        // Create multiple command files
        std::fs::write(commands_dir.join("greet.md"), "Hello, world!").unwrap();
        std::fs::write(commands_dir.join("review.md"), "Review: $1").unwrap();

        // Create a namespaced command
        let frontend_dir = commands_dir.join("frontend");
        std::fs::create_dir_all(&frontend_dir).unwrap();
        std::fs::write(frontend_dir.join("component.md"), "Component: $1").unwrap();

        let mut result = CommandLoadResult::default();
        super::load_commands_from_dir(commands_dir, commands_dir, CommandScope::User, &mut result);

        assert!(result.errors.is_empty());
        assert_eq!(result.commands.len(), 3);

        let names: Vec<&str> = result.commands.iter().map(|c| c.name.as_ref()).collect();
        assert!(names.contains(&"greet"));
        assert!(names.contains(&"review"));
        // Namespaced command has prefix
        assert!(names.contains(&"frontend:component"));

        let component_cmd = result
            .commands
            .iter()
            .find(|c| c.name.as_ref() == "frontend:component")
            .unwrap();
        assert_eq!(
            component_cmd.namespace.as_ref().map(|s| s.as_ref()),
            Some("frontend")
        );
    }

    #[test]
    fn test_load_commands_from_nonexistent_dir() {
        let temp_dir = tempfile::tempdir().unwrap();
        let nonexistent = temp_dir.path().join("does_not_exist");

        let mut result = CommandLoadResult::default();
        super::load_commands_from_dir(&nonexistent, &nonexistent, CommandScope::User, &mut result);
        assert!(result.errors.is_empty());
        assert!(result.commands.is_empty());
    }

    #[test]
    fn test_load_project_commands() {
        let temp_dir = tempfile::tempdir().unwrap();
        let worktree_root = temp_dir.path();

        // Create .zed/commands/ directory
        let commands_dir = worktree_root.join(".zed").join("commands");
        std::fs::create_dir_all(&commands_dir).unwrap();
        std::fs::write(commands_dir.join("build.md"), "Build the project").unwrap();

        let result = super::load_project_commands(worktree_root);
        assert!(result.errors.is_empty());
        assert_eq!(result.commands.len(), 1);
        assert_eq!(result.commands[0].name.as_ref(), "build");
        assert_eq!(result.commands[0].scope, CommandScope::Project);
        assert_eq!(result.commands[0].description(), "(project)");
    }

    #[test]
    fn test_load_all_commands_no_duplicates() {
        let temp_dir = tempfile::tempdir().unwrap();
        let project_root = temp_dir.path().join("project");
        std::fs::create_dir_all(&project_root).unwrap();

        // Create project command
        let project_commands = project_root.join(".zed").join("commands");
        std::fs::create_dir_all(&project_commands).unwrap();
        std::fs::write(project_commands.join("review.md"), "Project review: $1").unwrap();
        std::fs::write(project_commands.join("build.md"), "Build project").unwrap();

        // Create user command directory (we can't easily test this without mocking config_dir)
        // But we can test that project commands are loaded with correct scope
        let result = super::load_all_commands(std::slice::from_ref(&project_root));

        assert!(result.errors.is_empty());

        // Should have both project commands
        assert!(result.commands.iter().any(|c| c.name.as_ref() == "review"));
        assert!(result.commands.iter().any(|c| c.name.as_ref() == "build"));

        // All should be project scope
        for cmd in &result.commands {
            assert_eq!(cmd.scope, CommandScope::Project);
        }
    }

    #[test]
    fn test_load_all_commands_duplicate_error() {
        let temp_dir = tempfile::tempdir().unwrap();

        // Create two project roots with the same command name
        let project1 = temp_dir.path().join("project1");
        let project2 = temp_dir.path().join("project2");

        let commands1 = project1.join(".zed").join("commands");
        let commands2 = project2.join(".zed").join("commands");

        std::fs::create_dir_all(&commands1).unwrap();
        std::fs::create_dir_all(&commands2).unwrap();

        std::fs::write(commands1.join("deploy.md"), "Deploy from project1").unwrap();
        std::fs::write(commands2.join("deploy.md"), "Deploy from project2").unwrap();

        let result = super::load_all_commands(&[project1, project2]);

        // Should have one command loaded and one error for the duplicate
        assert_eq!(result.commands.len(), 1);
        assert_eq!(result.errors.len(), 1);
        assert!(result.errors[0].message.contains("ambiguous"));
        assert!(result.errors[0].message.contains("deploy"));
    }

    #[test]
    fn test_command_load_error_display() {
        let error = CommandLoadError {
            path: PathBuf::from("/path/to/command.md"),
            message: "Permission denied".to_string(),
        };
        assert_eq!(
            error.to_string(),
            "Failed to load /path/to/command.md: Permission denied"
        );
    }

    #[test]
    fn test_commands_to_map() {
        let commands = vec![
            UserSlashCommand {
                name: "greet".into(),
                template: "Hello!".into(),
                namespace: None,
                path: PathBuf::from("/greet.md"),
                scope: CommandScope::User,
            },
            UserSlashCommand {
                name: "review".into(),
                template: "Review: $1".into(),
                namespace: Some("code".into()),
                path: PathBuf::from("/code/review.md"),
                scope: CommandScope::User,
            },
        ];

        let map = commands_to_map(&commands);
        assert_eq!(map.len(), 2);
        assert!(map.contains_key("greet"));
        assert!(map.contains_key("review"));
        assert_eq!(map.get("greet").unwrap().template.as_ref(), "Hello!");
    }

    #[test]
    fn test_try_expand_from_commands() {
        let commands = vec![
            UserSlashCommand {
                name: "greet".into(),
                template: "Hello, world!".into(),
                namespace: None,
                path: PathBuf::from("/greet.md"),
                scope: CommandScope::User,
            },
            UserSlashCommand {
                name: "review".into(),
                template: "Review this for: $1".into(),
                namespace: None,
                path: PathBuf::from("/review.md"),
                scope: CommandScope::User,
            },
            UserSlashCommand {
                name: "search".into(),
                template: "Search: $ARGUMENTS".into(),
                namespace: None,
                path: PathBuf::from("/search.md"),
                scope: CommandScope::User,
            },
        ];
        let map = commands_to_map(&commands);

        // Test command without arguments
        let result = try_expand_from_commands("/greet", &map).unwrap();
        assert_eq!(result, Some("Hello, world!".to_string()));

        // Test command with positional argument
        let result = try_expand_from_commands("/review security", &map).unwrap();
        assert_eq!(result, Some("Review this for: security".to_string()));

        // Test command with $ARGUMENTS
        let result = try_expand_from_commands("/search foo bar baz", &map).unwrap();
        assert_eq!(result, Some("Search: foo bar baz".to_string()));

        // Test unknown command
        let result = try_expand_from_commands("/unknown", &map).unwrap();
        assert_eq!(result, None);

        // Test not a command
        let result = try_expand_from_commands("just text", &map).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn test_try_expand_from_commands_missing_args() {
        let commands = vec![UserSlashCommand {
            name: "review".into(),
            template: "Review: $1".into(),
            namespace: None,
            path: PathBuf::from("/review.md"),
            scope: CommandScope::User,
        }];
        let map = commands_to_map(&commands);

        let result = try_expand_from_commands("/review", &map);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("requires 1 argument")
        );
    }

    #[test]
    fn test_has_command() {
        let commands = vec![UserSlashCommand {
            name: "greet".into(),
            template: "Hello!".into(),
            namespace: None,
            path: PathBuf::from("/greet.md"),
            scope: CommandScope::User,
        }];
        let map = commands_to_map(&commands);

        assert!(has_command("greet", &map));
        assert!(!has_command("unknown", &map));
    }
}
