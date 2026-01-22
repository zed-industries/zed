use anyhow::{Result, anyhow};
use collections::{HashMap, HashSet};
use fs::Fs;
use futures::StreamExt;
use gpui::{Context, EventEmitter, Task};
use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

/// An error that occurred while loading a command file.
#[derive(Debug, Clone)]
pub struct CommandLoadError {
    /// The path to the file that failed to load
    pub path: PathBuf,
    /// The base path of the commands directory (used to derive command name)
    pub base_path: PathBuf,
    /// A description of the error
    pub message: String,
}

impl CommandLoadError {
    /// Derives the command name from the file path, similar to how successful commands are named.
    /// Returns None if the command name cannot be determined (e.g., for directory errors).
    pub fn command_name(&self) -> Option<String> {
        let base_name = self.path.file_stem()?.to_string_lossy().into_owned();

        // Only derive command name for .md files
        if self.path.extension().is_none_or(|ext| ext != "md") {
            return None;
        }

        let namespace = self
            .path
            .parent()
            .and_then(|parent| parent.strip_prefix(&self.base_path).ok())
            .filter(|rel| !rel.as_os_str().is_empty())
            .map(|rel| {
                rel.to_string_lossy()
                    .replace(std::path::MAIN_SEPARATOR, "/")
            });

        let name = match &namespace {
            Some(namespace) => format!("{}:{}", namespace.replace('/', ":"), base_name),
            None => base_name,
        };

        Some(name)
    }
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
#[derive(Debug, Default, Clone)]
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
    pub fn description(&self) -> String {
        String::new()
    }

    /// Returns true if this command has any placeholders ($1, $2, etc. or $ARGUMENTS)
    pub fn requires_arguments(&self) -> bool {
        has_placeholders(&self.template)
    }
}

fn command_base_path(command: &UserSlashCommand) -> PathBuf {
    let mut base_path = command.path.clone();
    base_path.pop();
    if let Some(namespace) = &command.namespace {
        for segment in namespace.split('/') {
            if segment.is_empty() {
                continue;
            }
            if !base_path.pop() {
                break;
            }
        }
    }
    base_path
}

impl CommandLoadError {
    pub fn from_command(command: &UserSlashCommand, message: String) -> Self {
        Self {
            path: command.path.clone(),
            base_path: command_base_path(command),
            message,
        }
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

/// Events emitted by SlashCommandRegistry
#[derive(Debug, Clone)]
#[allow(dead_code)] // Infrastructure for future caching implementation
pub enum SlashCommandRegistryEvent {
    /// Commands have been reloaded
    CommandsChanged,
}

/// A registry that caches user-defined slash commands and watches for changes.
/// Currently used in tests; will be integrated into the UI layer for caching.
#[allow(dead_code)]
pub struct SlashCommandRegistry {
    fs: Arc<dyn Fs>,
    commands: HashMap<String, UserSlashCommand>,
    errors: Vec<CommandLoadError>,
    worktree_roots: Vec<PathBuf>,
    _watch_task: Option<Task<()>>,
}

impl EventEmitter<SlashCommandRegistryEvent> for SlashCommandRegistry {}

#[allow(dead_code)]
impl SlashCommandRegistry {
    /// Creates a new registry and starts loading commands.
    pub fn new(fs: Arc<dyn Fs>, worktree_roots: Vec<PathBuf>, cx: &mut Context<Self>) -> Self {
        let mut this = Self {
            fs,
            commands: HashMap::default(),
            errors: Vec::new(),
            worktree_roots,
            _watch_task: None,
        };

        this.start_watching(cx);
        this.reload(cx);

        this
    }

    /// Returns all loaded commands.
    pub fn commands(&self) -> &HashMap<String, UserSlashCommand> {
        &self.commands
    }

    /// Returns any errors from the last load.
    pub fn errors(&self) -> &[CommandLoadError] {
        &self.errors
    }

    /// Updates the worktree roots and reloads commands.
    pub fn set_worktree_roots(&mut self, roots: Vec<PathBuf>, cx: &mut Context<Self>) {
        if self.worktree_roots != roots {
            self.worktree_roots = roots;
            self.start_watching(cx);
            self.reload(cx);
        }
    }

    /// Manually triggers a reload of all commands.
    pub fn reload(&mut self, cx: &mut Context<Self>) {
        let fs = self.fs.clone();
        let worktree_roots = self.worktree_roots.clone();

        cx.spawn(async move |this, cx| {
            let result = load_all_commands_async(&fs, &worktree_roots).await;
            this.update(cx, |this, cx| {
                this.commands = commands_to_map(&result.commands);
                this.errors = result.errors;
                cx.emit(SlashCommandRegistryEvent::CommandsChanged);
            })
        })
        .detach_and_log_err(cx);
    }

    fn start_watching(&mut self, cx: &mut Context<Self>) {
        let fs = self.fs.clone();
        let worktree_roots = self.worktree_roots.clone();

        let task = cx.spawn(async move |this, cx| {
            let user_dir = user_commands_dir();
            let mut dirs_to_watch = vec![user_dir];
            for root in &worktree_roots {
                dirs_to_watch.push(project_commands_dir(root));
            }

            let mut watch_streams = Vec::new();
            for dir in &dirs_to_watch {
                let (stream, _watcher) = fs.watch(dir, Duration::from_millis(100)).await;
                watch_streams.push(stream);
            }

            let mut combined = futures::stream::select_all(watch_streams);

            while let Some(events) = combined.next().await {
                let should_reload = events.iter().any(|event| {
                    event.path.extension().is_some_and(|ext| ext == "md")
                        || event.kind == Some(fs::PathEventKind::Created)
                        || event.kind == Some(fs::PathEventKind::Removed)
                });

                if should_reload {
                    let result = load_all_commands_async(&fs, &worktree_roots).await;
                    let _ = this.update(cx, |this, cx| {
                        this.commands = commands_to_map(&result.commands);
                        this.errors = result.errors;
                        cx.emit(SlashCommandRegistryEvent::CommandsChanged);
                    });
                }
            }
        });

        self._watch_task = Some(task);
    }
}

/// Loads all commands (both project and user) for given worktree roots asynchronously.
pub async fn load_all_commands_async(
    fs: &Arc<dyn Fs>,
    worktree_roots: &[PathBuf],
) -> CommandLoadResult {
    let mut result = CommandLoadResult::default();
    let mut seen_commands: HashMap<String, PathBuf> = HashMap::default();

    // Load project commands first
    for root in worktree_roots {
        let commands_path = project_commands_dir(root);
        let project_result =
            load_commands_from_path_async(fs, &commands_path, CommandScope::Project).await;
        result.errors.extend(project_result.errors);
        for cmd in project_result.commands {
            if let Some(existing_path) = seen_commands.get(&*cmd.name) {
                result.errors.push(CommandLoadError {
                    path: cmd.path.clone(),
                    base_path: commands_path.clone(),
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
    let user_commands_path = user_commands_dir();
    let user_result =
        load_commands_from_path_async(fs, &user_commands_path, CommandScope::User).await;
    result.errors.extend(user_result.errors);
    for cmd in user_result.commands {
        if let Some(existing_path) = seen_commands.get(&*cmd.name) {
            result.errors.push(CommandLoadError {
                path: cmd.path.clone(),
                base_path: user_commands_path.clone(),
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

async fn load_commands_from_path_async(
    fs: &Arc<dyn Fs>,
    commands_path: &Path,
    scope: CommandScope,
) -> CommandLoadResult {
    let mut result = CommandLoadResult::default();

    if !fs.is_dir(commands_path).await {
        return result;
    }

    load_commands_from_dir_async(fs, commands_path, commands_path, scope, &mut result).await;
    result
}

fn load_commands_from_dir_async<'a>(
    fs: &'a Arc<dyn Fs>,
    base_path: &'a Path,
    current_path: &'a Path,
    scope: CommandScope,
    result: &'a mut CommandLoadResult,
) -> futures::future::BoxFuture<'a, ()> {
    Box::pin(async move {
        let entries = match fs.read_dir(current_path).await {
            Ok(entries) => entries,
            Err(e) => {
                result.errors.push(CommandLoadError {
                    path: current_path.to_path_buf(),
                    base_path: base_path.to_path_buf(),
                    message: format!("Failed to read directory: {}", e),
                });
                return;
            }
        };

        let entries: Vec<_> = entries.collect().await;

        for entry in entries {
            let path = match entry {
                Ok(path) => path,
                Err(e) => {
                    result.errors.push(CommandLoadError {
                        path: current_path.to_path_buf(),
                        base_path: base_path.to_path_buf(),
                        message: format!("Failed to read directory entry: {}", e),
                    });
                    continue;
                }
            };

            if fs.is_dir(&path).await {
                load_commands_from_dir_async(fs, base_path, &path, scope, result).await;
            } else if path.extension().is_some_and(|ext| ext == "md") {
                match load_command_file_async(fs, base_path, &path, scope).await {
                    Ok(Some(command)) => result.commands.push(command),
                    Ok(None) => {} // Empty file, skip silently
                    Err(e) => {
                        result.errors.push(CommandLoadError {
                            path: path.clone(),
                            base_path: base_path.to_path_buf(),
                            message: e.to_string(),
                        });
                    }
                }
            }
        }
    })
}

async fn load_command_file_async(
    fs: &Arc<dyn Fs>,
    base_path: &Path,
    file_path: &Path,
    scope: CommandScope,
) -> Result<Option<UserSlashCommand>> {
    let base_name = match file_path.file_stem() {
        Some(stem) => stem.to_string_lossy().into_owned(),
        None => return Ok(None),
    };

    let template = fs.load(file_path).await?;
    if template.is_empty() {
        return Ok(None);
    }
    if template.trim().is_empty() {
        return Err(anyhow!("Command file contains only whitespace"));
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
        Some(namespace) => format!("{}:{}", namespace.replace('/', ":"), base_name),
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

fn has_error_for_command(errors: &[CommandLoadError], name: &str) -> bool {
    errors
        .iter()
        .any(|error| error.command_name().as_deref() == Some(name))
}

fn server_conflict_message(name: &str) -> String {
    format!(
        "Command '{}' conflicts with server-provided /{}",
        name, name
    )
}

pub fn apply_server_command_conflicts(
    commands: &mut Vec<UserSlashCommand>,
    errors: &mut Vec<CommandLoadError>,
    server_command_names: &HashSet<String>,
) {
    commands.retain(|command| {
        if server_command_names.contains(command.name.as_ref()) {
            if !has_error_for_command(errors, command.name.as_ref()) {
                errors.push(CommandLoadError::from_command(
                    command,
                    server_conflict_message(command.name.as_ref()),
                ));
            }
            false
        } else {
            true
        }
    });
}

pub fn apply_server_command_conflicts_to_map(
    commands: &mut HashMap<String, UserSlashCommand>,
    errors: &mut Vec<CommandLoadError>,
    server_command_names: &HashSet<String>,
) {
    commands.retain(|name, command| {
        if server_command_names.contains(name) {
            if !has_error_for_command(errors, name) {
                errors.push(CommandLoadError::from_command(
                    command,
                    server_conflict_message(name),
                ));
            }
            false
        } else {
            true
        }
    });
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
    validate_arguments(command_name, template, arguments)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use fs::{FakeFs, Fs, RemoveOptions};
    use gpui::{AppContext as _, TestAppContext};
    use serde_json::json;
    use std::sync::Arc;
    use text::Rope;
    use util::path;

    // ==================== Parsing Tests ====================

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

    // ==================== Placeholder Tests ====================

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

    // ==================== Template Expansion Tests ====================

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
    fn test_expand_template_escape_sequences() {
        let args: Vec<Cow<'_, str>> = vec![];
        assert_eq!(
            expand_template("line1\\nline2", &args, "").unwrap(),
            "line1\nline2"
        );
        assert_eq!(
            expand_template("cost is \\$1", &args, "").unwrap(),
            "cost is $1"
        );
        assert_eq!(
            expand_template("say \\\"hi\\\"", &args, "").unwrap(),
            "say \"hi\""
        );
        assert_eq!(
            expand_template("path\\\\file", &args, "").unwrap(),
            "path\\file"
        );
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

    // ==================== Validation Tests ====================

    #[test]
    fn test_validate_arguments_exact_match() {
        let args = vec![Cow::Borrowed("a"), Cow::Borrowed("b")];
        let result = validate_arguments("test", "$1 $2", &args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_arguments_missing_args() {
        let args = vec![Cow::Borrowed("a")];
        let result = validate_arguments("foo", "$1 $2", &args);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("/foo"));
        assert!(err.contains("requires 2 arguments"));
    }

    #[test]
    fn test_validate_arguments_extra_args() {
        let args = vec![Cow::Borrowed("a"), Cow::Borrowed("b")];
        let result = validate_arguments("foo", "$1", &args);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("accepts 1 argument"));
    }

    #[test]
    fn test_validate_arguments_no_placeholders() {
        // No args expected, none provided - OK
        let args: Vec<Cow<'_, str>> = vec![];
        assert!(validate_arguments("test", "no placeholders", &args).is_ok());

        // No args expected but some provided - Error
        let args = vec![Cow::Borrowed("unexpected")];
        let result = validate_arguments("test", "no placeholders", &args);
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
        let result = validate_arguments("test", "", &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_validate_arguments_with_arguments_placeholder() {
        // $ARGUMENTS accepts any number of arguments including zero
        let args: Vec<Cow<'_, str>> = vec![];
        assert!(validate_arguments("test", "Do: $ARGUMENTS", &args).is_ok());

        let args = vec![Cow::Borrowed("a"), Cow::Borrowed("b"), Cow::Borrowed("c")];
        assert!(validate_arguments("test", "Do: $ARGUMENTS", &args).is_ok());
    }

    #[test]
    fn test_validate_arguments_mixed_placeholders() {
        // Both $ARGUMENTS and positional - need at least the positional ones
        let args = vec![Cow::Borrowed("first")];
        assert!(validate_arguments("test", "$1 then $ARGUMENTS", &args).is_ok());

        let args: Vec<Cow<'_, str>> = vec![];
        assert!(validate_arguments("test", "$1 then $ARGUMENTS", &args).is_err());
    }

    // ==================== Integration Tests ====================

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

        // Command without arguments
        assert_eq!(
            try_expand_from_commands("/greet", &map).unwrap(),
            Some("Hello, world!".to_string())
        );

        // Command with positional argument
        assert_eq!(
            try_expand_from_commands("/review security", &map).unwrap(),
            Some("Review this for: security".to_string())
        );

        // Command with $ARGUMENTS
        assert_eq!(
            try_expand_from_commands("/search foo bar baz", &map).unwrap(),
            Some("Search: foo bar baz".to_string())
        );

        // Unknown command returns None
        assert_eq!(try_expand_from_commands("/unknown", &map).unwrap(), None);

        // Not a command returns None
        assert_eq!(try_expand_from_commands("just text", &map).unwrap(), None);
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

    // ==================== Edge Case Tests ====================

    #[test]
    fn test_unicode_command_names() {
        // Test that unicode in command names works
        let result = try_parse_user_command("/日本語 arg1");
        assert!(result.is_some());
        let parsed = result.unwrap();
        assert_eq!(parsed.name, "日本語");
        assert_eq!(parsed.raw_arguments, "arg1");
    }

    #[test]
    fn test_unicode_in_arguments() {
        let args = parse_arguments("\"こんにちは\" 世界").unwrap();
        assert_eq!(args, vec!["こんにちは", "世界"]);
    }

    #[test]
    fn test_unicode_in_template() {
        let args = vec![Cow::Borrowed("名前")];
        let result = expand_template("こんにちは、$1さん！", &args, "名前").unwrap();
        assert_eq!(result, "こんにちは、名前さん！");
    }

    #[test]
    fn test_command_name_with_emoji() {
        // Emoji can be multi-codepoint, test they're handled correctly
        let result = try_parse_user_command("/🚀deploy fast");
        assert!(result.is_some());
        let parsed = result.unwrap();
        assert_eq!(parsed.name, "🚀deploy");
        assert_eq!(parsed.raw_arguments, "fast");

        // Emoji in arguments
        let args = parse_arguments("🎉 \"🎊 party\"").unwrap();
        assert_eq!(args, vec!["🎉", "🎊 party"]);
    }

    #[test]
    fn test_many_placeholders() {
        // Test template with many placeholders
        let template = "$1 $2 $3 $4 $5 $6 $7 $8 $9 $10";
        assert_eq!(count_positional_placeholders(template), 10);

        let args: Vec<Cow<'_, str>> = (1..=10).map(|i| Cow::Owned(i.to_string())).collect();
        let result = expand_template(template, &args, "1 2 3 4 5 6 7 8 9 10").unwrap();
        assert_eq!(result, "1 2 3 4 5 6 7 8 9 10");
    }

    #[test]
    fn test_placeholder_zero_is_invalid() {
        let args = vec![Cow::Borrowed("a")];
        let result = expand_template("$0", &args, "a");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("$0 is invalid"));
    }

    #[test]
    fn test_dollar_sign_without_number() {
        // Bare $ should be preserved
        let args: Vec<Cow<'_, str>> = vec![];
        let result = expand_template("cost is $", &args, "").unwrap();
        assert_eq!(result, "cost is $");
    }

    #[test]
    fn test_consecutive_whitespace_in_arguments() {
        let args = parse_arguments("  a    b   c  ").unwrap();
        assert_eq!(args, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_empty_input() {
        let args = parse_arguments("").unwrap();
        assert!(args.is_empty());

        let args = parse_arguments("   ").unwrap();
        assert!(args.is_empty());
    }

    #[test]
    fn test_command_load_error_command_name() {
        let error = CommandLoadError {
            path: PathBuf::from(path!("/commands/tools/git/commit.md")),
            base_path: PathBuf::from(path!("/commands")),
            message: "Failed".into(),
        };
        assert_eq!(error.command_name().as_deref(), Some("tools:git:commit"));

        let non_md_error = CommandLoadError {
            path: PathBuf::from(path!("/commands/readme.txt")),
            base_path: PathBuf::from(path!("/commands")),
            message: "Failed".into(),
        };
        assert_eq!(non_md_error.command_name(), None);
    }

    #[test]
    fn test_apply_server_command_conflicts() {
        let mut commands = vec![
            UserSlashCommand {
                name: "help".into(),
                template: "Help text".into(),
                namespace: None,
                path: PathBuf::from(path!("/commands/help.md")),
                scope: CommandScope::User,
            },
            UserSlashCommand {
                name: "review".into(),
                template: "Review $1".into(),
                namespace: None,
                path: PathBuf::from(path!("/commands/review.md")),
                scope: CommandScope::User,
            },
        ];
        let mut errors = Vec::new();
        let server_command_names = HashSet::from_iter(["help".to_string()]);

        apply_server_command_conflicts(&mut commands, &mut errors, &server_command_names);

        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].name.as_ref(), "review");
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].command_name().as_deref(), Some("help"));
        assert!(errors[0].message.contains("conflicts"));
    }

    #[test]
    fn test_apply_server_command_conflicts_to_map() {
        let command = UserSlashCommand {
            name: "tools:git:commit".into(),
            template: "Commit".into(),
            namespace: Some("tools/git".into()),
            path: PathBuf::from(path!("/commands/tools/git/commit.md")),
            scope: CommandScope::User,
        };
        let mut commands = HashMap::default();
        commands.insert(command.name.to_string(), command.clone());
        let mut errors = Vec::new();
        let server_command_names = HashSet::from_iter([command.name.to_string()]);

        apply_server_command_conflicts_to_map(&mut commands, &mut errors, &server_command_names);

        assert!(commands.is_empty());
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].command_name().as_deref(),
            Some("tools:git:commit")
        );
    }

    // ==================== Async File Loading Tests with FakeFs ====================

    #[gpui::test]
    async fn test_load_commands_from_empty_dir(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/commands"), json!({})).await;
        let fs: Arc<dyn Fs> = fs;

        let result =
            load_commands_from_path_async(&fs, Path::new(path!("/commands")), CommandScope::User)
                .await;

        assert!(result.commands.is_empty());
        assert!(result.errors.is_empty());
    }

    #[gpui::test]
    async fn test_load_commands_from_nonexistent_dir(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/"), json!({})).await;
        let fs: Arc<dyn Fs> = fs;

        let result = load_commands_from_path_async(
            &fs,
            Path::new(path!("/nonexistent")),
            CommandScope::User,
        )
        .await;

        assert!(result.commands.is_empty());
        assert!(result.errors.is_empty());
    }

    #[gpui::test]
    async fn test_load_single_command(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/commands"),
            json!({
                "review.md": "Please review: $1"
            }),
        )
        .await;
        let fs: Arc<dyn Fs> = fs;

        let result =
            load_commands_from_path_async(&fs, Path::new(path!("/commands")), CommandScope::User)
                .await;

        assert!(result.errors.is_empty());
        assert_eq!(result.commands.len(), 1);
        let cmd = &result.commands[0];
        assert_eq!(cmd.name.as_ref(), "review");
        assert_eq!(cmd.template.as_ref(), "Please review: $1");
        assert!(cmd.namespace.is_none());
        assert_eq!(cmd.scope, CommandScope::User);
    }

    #[gpui::test]
    async fn test_load_commands_with_namespace(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/commands"),
            json!({
                "frontend": {
                    "component.md": "Create component: $1"
                }
            }),
        )
        .await;
        let fs: Arc<dyn Fs> = fs;

        let result =
            load_commands_from_path_async(&fs, Path::new(path!("/commands")), CommandScope::User)
                .await;

        assert!(result.errors.is_empty());
        assert_eq!(result.commands.len(), 1);
        let cmd = &result.commands[0];
        assert_eq!(cmd.name.as_ref(), "frontend:component");
        assert_eq!(cmd.namespace.as_ref().map(|s| s.as_ref()), Some("frontend"));
    }

    #[gpui::test]
    async fn test_load_commands_nested_namespace(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/commands"),
            json!({
                "tools": {
                    "git": {
                        "commit.md": "Git commit: $ARGUMENTS"
                    }
                }
            }),
        )
        .await;
        let fs: Arc<dyn Fs> = fs;

        let result =
            load_commands_from_path_async(&fs, Path::new(path!("/commands")), CommandScope::User)
                .await;

        assert!(result.errors.is_empty());
        assert_eq!(result.commands.len(), 1);
        let cmd = &result.commands[0];
        assert_eq!(cmd.name.as_ref(), "tools:git:commit");
        assert_eq!(
            cmd.namespace.as_ref().map(|s| s.as_ref()),
            Some("tools/git")
        );
    }

    #[gpui::test]
    async fn test_deeply_nested_namespace(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/commands"),
            json!({
                "a": {
                    "b": {
                        "c": {
                            "d": {
                                "e": {
                                    "deep.md": "Very deep command"
                                }
                            }
                        }
                    }
                }
            }),
        )
        .await;
        let fs: Arc<dyn Fs> = fs;

        let result =
            load_commands_from_path_async(&fs, Path::new(path!("/commands")), CommandScope::User)
                .await;

        assert!(result.errors.is_empty());
        assert_eq!(result.commands.len(), 1);
        let cmd = &result.commands[0];
        assert_eq!(cmd.name.as_ref(), "a:b:c:d:e:deep");
        assert_eq!(
            cmd.namespace.as_ref().map(|s| s.as_ref()),
            Some("a/b/c/d/e")
        );
    }

    #[gpui::test]
    async fn test_load_commands_empty_file_ignored(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/commands"),
            json!({
                "empty.md": "",
                "valid.md": "Hello!"
            }),
        )
        .await;
        let fs: Arc<dyn Fs> = fs;

        let result =
            load_commands_from_path_async(&fs, Path::new(path!("/commands")), CommandScope::User)
                .await;

        assert!(result.errors.is_empty());
        assert_eq!(result.commands.len(), 1);
        assert_eq!(result.commands[0].name.as_ref(), "valid");
    }

    #[gpui::test]
    async fn test_load_commands_non_md_files_ignored(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/commands"),
            json!({
                "command.md": "Valid command",
                "readme.txt": "Not a command",
                "script.sh": "Also not a command"
            }),
        )
        .await;
        let fs: Arc<dyn Fs> = fs;

        let result =
            load_commands_from_path_async(&fs, Path::new(path!("/commands")), CommandScope::User)
                .await;

        assert!(result.errors.is_empty());
        assert_eq!(result.commands.len(), 1);
        assert_eq!(result.commands[0].name.as_ref(), "command");
    }

    #[gpui::test]
    async fn test_load_project_commands(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".zed": {
                    "commands": {
                        "build.md": "Build the project"
                    }
                }
            }),
        )
        .await;
        let fs: Arc<dyn Fs> = fs;

        let commands_path = project_commands_dir(Path::new(path!("/project")));
        let result =
            load_commands_from_path_async(&fs, &commands_path, CommandScope::Project).await;

        assert!(result.errors.is_empty());
        assert_eq!(result.commands.len(), 1);
        assert_eq!(result.commands[0].name.as_ref(), "build");
        assert_eq!(result.commands[0].scope, CommandScope::Project);
    }

    #[gpui::test]
    async fn test_load_all_commands_no_duplicates(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project1"),
            json!({
                ".zed": {
                    "commands": {
                        "review.md": "Project 1 review"
                    }
                }
            }),
        )
        .await;
        fs.insert_tree(
            path!("/project2"),
            json!({
                ".zed": {
                    "commands": {
                        "build.md": "Project 2 build"
                    }
                }
            }),
        )
        .await;
        let fs: Arc<dyn Fs> = fs;

        let result = load_all_commands_async(
            &fs,
            &[
                PathBuf::from(path!("/project1")),
                PathBuf::from(path!("/project2")),
            ],
        )
        .await;

        assert!(result.errors.is_empty());
        assert_eq!(result.commands.len(), 2);
        let names: Vec<&str> = result.commands.iter().map(|c| c.name.as_ref()).collect();
        assert!(names.contains(&"review"));
        assert!(names.contains(&"build"));
    }

    #[gpui::test]
    async fn test_load_all_commands_duplicate_error(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project1"),
            json!({
                ".zed": {
                    "commands": {
                        "deploy.md": "Deploy from project 1"
                    }
                }
            }),
        )
        .await;
        fs.insert_tree(
            path!("/project2"),
            json!({
                ".zed": {
                    "commands": {
                        "deploy.md": "Deploy from project 2"
                    }
                }
            }),
        )
        .await;
        let fs: Arc<dyn Fs> = fs;

        let result = load_all_commands_async(
            &fs,
            &[
                PathBuf::from(path!("/project1")),
                PathBuf::from(path!("/project2")),
            ],
        )
        .await;

        // Should have one command and one error
        assert_eq!(result.commands.len(), 1);
        assert_eq!(result.errors.len(), 1);
        assert!(result.errors[0].message.contains("ambiguous"));
        assert!(result.errors[0].message.contains("deploy"));
    }

    #[gpui::test]
    async fn test_registry_loads_commands(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".zed": {
                    "commands": {
                        "test.md": "Test command"
                    }
                }
            }),
        )
        .await;
        let fs: Arc<dyn Fs> = fs;

        let registry = cx.new(|cx| {
            SlashCommandRegistry::new(fs.clone(), vec![PathBuf::from(path!("/project"))], cx)
        });

        // Wait for async load
        cx.run_until_parked();

        registry.read_with(cx, |registry: &SlashCommandRegistry, _cx| {
            assert!(registry.errors().is_empty());
            assert!(registry.commands().contains_key("test"));
        });
    }

    #[gpui::test]
    async fn test_registry_updates_worktree_roots(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project1"),
            json!({
                ".zed": {
                    "commands": {
                        "cmd1.md": "Command 1"
                    }
                }
            }),
        )
        .await;
        fs.insert_tree(
            path!("/project2"),
            json!({
                ".zed": {
                    "commands": {
                        "cmd2.md": "Command 2"
                    }
                }
            }),
        )
        .await;
        let fs: Arc<dyn Fs> = fs;

        let registry = cx.new(|cx| {
            SlashCommandRegistry::new(fs.clone(), vec![PathBuf::from(path!("/project1"))], cx)
        });

        cx.run_until_parked();

        registry.read_with(cx, |registry: &SlashCommandRegistry, _cx| {
            assert!(registry.commands().contains_key("cmd1"));
            assert!(!registry.commands().contains_key("cmd2"));
        });

        // Update worktree roots
        registry.update(cx, |registry: &mut SlashCommandRegistry, cx| {
            registry.set_worktree_roots(
                vec![
                    PathBuf::from(path!("/project1")),
                    PathBuf::from(path!("/project2")),
                ],
                cx,
            );
        });

        cx.run_until_parked();

        registry.read_with(cx, |registry: &SlashCommandRegistry, _cx| {
            assert!(registry.commands().contains_key("cmd1"));
            assert!(registry.commands().contains_key("cmd2"));
        });
    }

    #[gpui::test]
    async fn test_registry_reloads_on_file_change(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".zed": {
                    "commands": {
                        "original.md": "Original command"
                    }
                }
            }),
        )
        .await;
        let fs: Arc<dyn Fs> = fs.clone();

        let registry = cx.new(|cx| {
            SlashCommandRegistry::new(fs.clone(), vec![PathBuf::from(path!("/project"))], cx)
        });

        // Wait for initial load
        cx.run_until_parked();

        registry.read_with(cx, |registry, _cx| {
            assert_eq!(registry.commands().len(), 1);
            assert!(registry.commands().contains_key("original"));
        });

        // Add a new command file
        fs.save(
            Path::new(path!("/project/.zed/commands/new.md")),
            &Rope::from("New command"),
            text::LineEnding::Unix,
        )
        .await
        .unwrap();

        // Wait for watcher to process the change
        cx.run_until_parked();

        registry.read_with(cx, |registry, _cx| {
            assert_eq!(registry.commands().len(), 2);
            assert!(registry.commands().contains_key("original"));
            assert!(registry.commands().contains_key("new"));
        });

        // Remove a command file
        fs.remove_file(
            Path::new(path!("/project/.zed/commands/original.md")),
            RemoveOptions::default(),
        )
        .await
        .unwrap();

        // Wait for watcher to process the change
        cx.run_until_parked();

        registry.read_with(cx, |registry, _cx| {
            assert_eq!(registry.commands().len(), 1);
            assert!(!registry.commands().contains_key("original"));
            assert!(registry.commands().contains_key("new"));
        });

        // Modify an existing command
        fs.save(
            Path::new(path!("/project/.zed/commands/new.md")),
            &Rope::from("Updated content"),
            text::LineEnding::Unix,
        )
        .await
        .unwrap();

        cx.run_until_parked();

        registry.read_with(cx, |registry, _cx| {
            let cmd = registry.commands().get("new").unwrap();
            assert_eq!(cmd.template.as_ref(), "Updated content");
        });
    }

    #[gpui::test]
    async fn test_concurrent_command_loading(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".zed": {
                    "commands": {
                        "cmd1.md": "Command 1",
                        "cmd2.md": "Command 2",
                        "cmd3.md": "Command 3"
                    }
                }
            }),
        )
        .await;
        let fs: Arc<dyn Fs> = fs;
        let worktree_roots = vec![PathBuf::from(path!("/project"))];

        // Spawn multiple load tasks concurrently
        let fs1 = fs.clone();
        let roots1 = worktree_roots.clone();
        let task1 = cx
            .executor()
            .spawn(async move { load_all_commands_async(&fs1, &roots1).await });

        let fs2 = fs.clone();
        let roots2 = worktree_roots.clone();
        let task2 = cx
            .executor()
            .spawn(async move { load_all_commands_async(&fs2, &roots2).await });

        let fs3 = fs.clone();
        let roots3 = worktree_roots.clone();
        let task3 = cx
            .executor()
            .spawn(async move { load_all_commands_async(&fs3, &roots3).await });

        // Wait for all tasks to complete
        let (result1, result2, result3) = futures::join!(task1, task2, task3);

        // All should succeed with the same results
        assert!(result1.errors.is_empty());
        assert!(result2.errors.is_empty());
        assert!(result3.errors.is_empty());

        assert_eq!(result1.commands.len(), 3);
        assert_eq!(result2.commands.len(), 3);
        assert_eq!(result3.commands.len(), 3);
    }

    // ==================== Symlink Handling Tests ====================

    #[gpui::test]
    async fn test_load_commands_from_symlinked_directory(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());

        // Create the actual commands directory with a command
        fs.insert_tree(
            path!("/actual_commands"),
            json!({
                "review.md": "Please review: $1"
            }),
        )
        .await;

        // Create a symlink from /commands to /actual_commands
        fs.insert_tree(path!("/"), json!({})).await;
        fs.create_symlink(
            Path::new(path!("/commands")),
            PathBuf::from(path!("/actual_commands")),
        )
        .await
        .unwrap();

        let fs: Arc<dyn Fs> = fs;

        let result =
            load_commands_from_path_async(&fs, Path::new(path!("/commands")), CommandScope::User)
                .await;

        assert!(result.errors.is_empty());
        assert_eq!(result.commands.len(), 1);
        assert_eq!(result.commands[0].name.as_ref(), "review");
    }

    #[gpui::test]
    async fn test_load_commands_from_symlinked_file(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());

        // Create the actual command file
        fs.insert_tree(
            path!("/actual"),
            json!({
                "real_review.md": "Review command content: $1"
            }),
        )
        .await;

        // Create commands directory with a symlink to the file
        fs.insert_tree(path!("/commands"), json!({})).await;
        fs.create_symlink(
            Path::new(path!("/commands/review.md")),
            PathBuf::from(path!("/actual/real_review.md")),
        )
        .await
        .unwrap();

        let fs: Arc<dyn Fs> = fs;

        let result =
            load_commands_from_path_async(&fs, Path::new(path!("/commands")), CommandScope::User)
                .await;

        assert!(result.errors.is_empty());
        assert_eq!(result.commands.len(), 1);
        assert_eq!(result.commands[0].name.as_ref(), "review");
        assert_eq!(
            result.commands[0].template.as_ref(),
            "Review command content: $1"
        );
    }

    #[gpui::test]
    async fn test_load_commands_claude_symlink_pattern(cx: &mut TestAppContext) {
        // Simulates the common pattern of symlinking ~/.claude/commands/ to zed's commands dir
        let fs = FakeFs::new(cx.executor());

        // Create Claude's commands directory structure
        fs.insert_tree(
            path!("/home/user/.claude/commands"),
            json!({
                "explain.md": "Explain this code: $ARGUMENTS",
                "refactor": {
                    "extract.md": "Extract method: $1"
                }
            }),
        )
        .await;

        // Create Zed config dir with symlink to Claude's commands
        fs.insert_tree(path!("/home/user/.config/zed"), json!({}))
            .await;
        fs.create_symlink(
            Path::new(path!("/home/user/.config/zed/commands")),
            PathBuf::from(path!("/home/user/.claude/commands")),
        )
        .await
        .unwrap();

        let fs: Arc<dyn Fs> = fs;

        let result = load_commands_from_path_async(
            &fs,
            Path::new(path!("/home/user/.config/zed/commands")),
            CommandScope::User,
        )
        .await;

        assert!(result.errors.is_empty());
        assert_eq!(result.commands.len(), 2);

        let names: Vec<&str> = result.commands.iter().map(|c| c.name.as_ref()).collect();
        assert!(names.contains(&"explain"));
        assert!(names.contains(&"refactor:extract"));
    }

    #[gpui::test]
    async fn test_symlink_to_parent_directory_skipped(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());

        // Create a directory structure with a symlink pointing outside the commands dir
        // This tests that symlinks to directories outside the command tree are handled
        fs.insert_tree(
            path!("/commands"),
            json!({
                "valid.md": "Valid command"
            }),
        )
        .await;

        // Create a separate directory
        fs.insert_tree(
            path!("/other"),
            json!({
                "external.md": "External command"
            }),
        )
        .await;

        // Create a symlink from /commands/external -> /other
        fs.create_symlink(
            Path::new(path!("/commands/external")),
            PathBuf::from(path!("/other")),
        )
        .await
        .unwrap();

        let fs: Arc<dyn Fs> = fs;

        let result =
            load_commands_from_path_async(&fs, Path::new(path!("/commands")), CommandScope::User)
                .await;

        // Should have loaded both the valid command and the external one via symlink
        assert!(result.commands.iter().any(|c| c.name.as_ref() == "valid"));
        assert!(
            result
                .commands
                .iter()
                .any(|c| c.name.as_ref() == "external:external")
        );
    }

    // ==================== Permission/Error Handling Tests ====================

    #[gpui::test]
    async fn test_load_commands_reports_directory_read_errors(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());

        // Create base directory but no commands subdirectory
        fs.insert_tree(path!("/"), json!({})).await;

        let fs: Arc<dyn Fs> = fs;

        // Try to load from a path that exists but isn't a directory
        // First create a file where we expect a directory
        fs.create_file(Path::new(path!("/commands")), fs::CreateOptions::default())
            .await
            .unwrap();

        let result =
            load_commands_from_path_async(&fs, Path::new(path!("/commands")), CommandScope::User)
                .await;

        // Should return empty since /commands is a file, not a directory
        assert!(result.commands.is_empty());
    }

    #[gpui::test]
    async fn test_load_all_commands_aggregates_errors(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());

        // Create two projects with duplicate command names
        fs.insert_tree(
            path!("/project1"),
            json!({
                ".zed": {
                    "commands": {
                        "build.md": "Build 1"
                    }
                }
            }),
        )
        .await;
        fs.insert_tree(
            path!("/project2"),
            json!({
                ".zed": {
                    "commands": {
                        "build.md": "Build 2"
                    }
                }
            }),
        )
        .await;
        fs.insert_tree(
            path!("/project3"),
            json!({
                ".zed": {
                    "commands": {
                        "build.md": "Build 3"
                    }
                }
            }),
        )
        .await;

        let fs: Arc<dyn Fs> = fs;

        let result = load_all_commands_async(
            &fs,
            &[
                PathBuf::from(path!("/project1")),
                PathBuf::from(path!("/project2")),
                PathBuf::from(path!("/project3")),
            ],
        )
        .await;

        // Should have 1 command (first one) and 2 errors (for duplicates)
        assert_eq!(result.commands.len(), 1);
        assert_eq!(result.errors.len(), 2);

        // All errors should mention "ambiguous"
        for error in &result.errors {
            assert!(error.message.contains("ambiguous"));
        }
    }

    #[gpui::test]
    async fn test_mixed_valid_and_empty_files(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());

        fs.insert_tree(
            path!("/commands"),
            json!({
                "valid.md": "Valid command",
                "empty.md": "",
                "whitespace_only.md": "   ",
                "another_valid.md": "Another valid"
            }),
        )
        .await;

        let fs: Arc<dyn Fs> = fs;

        let result =
            load_commands_from_path_async(&fs, Path::new(path!("/commands")), CommandScope::User)
                .await;

        // Empty file is ignored, whitespace-only is an error
        assert_eq!(result.commands.len(), 2);
        assert_eq!(result.errors.len(), 1);
        assert!(result.errors[0].message.contains("whitespace"));
        assert_eq!(
            result.errors[0].command_name().as_deref(),
            Some("whitespace_only")
        );
    }
}
