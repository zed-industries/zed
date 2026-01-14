# User-Defined Slash Commands

## Status: ✅ COMPLETE (File-based system implemented)

## TODO

### Error Handling UI ✅ COMPLETE
Implemented in `crates/agent_ui/src/acp/thread_view.rs`:
- Added `command_load_errors: Vec<CommandLoadError>` and `command_load_errors_dismissed: bool` fields to `AcpThreadView`
- Commands are loaded on thread view initialization when the `UserSlashCommandsFeatureFlag` is enabled
- Added `render_command_load_errors()` which displays a dismissable `Callout` with severity `Warning`
- Single error shows "Failed to load slash command", multiple errors shows "Failed to load N slash commands"
- All errors are listed in the description with bullet points: "• {path}: {message}"
- Added `clear_command_load_errors()` to handle dismissal

### Ambiguous Commands
If multiple commands have the same name (e.g., same command in two different project worktrees, or a project command and user command with the same name), an error is reported to the user. There is no silent precedence - the user must resolve the ambiguity by renaming one of the commands.

## Development Approach

- **Commit incrementally**: Commit working code after each phase or significant milestone
- **Test extensively**: The parser and argument handling have many edge cases; comprehensive unit tests are critical

## Overview

This feature allows users to define custom slash commands as Markdown files that expand into templated text when used in the agent panel. Commands are stored in `config_dir()/commands/` (e.g., `~/.config/zed/commands/` on macOS).

This follows the same conventions as Claude Code's slash commands, allowing users to symlink their `~/.claude/commands/` directory to Zed's commands directory for compatibility.

## User Experience

### Configuration

Users create Markdown files in their Zed config commands directory:

```
~/.config/zed/commands/
├── review.md           # Creates /review command
├── explain.md          # Creates /explain command
└── frontend/           # Namespace: commands show as "(user:frontend)"
    └── component.md    # Creates /component command
```

**Example command file (`review.md`):**
```markdown
Please review this code for correctness, performance, and style. Focus on: $1
```

**Example with $ARGUMENTS (`search.md`):**
```markdown
Search the codebase for: $ARGUMENTS
```

### Usage

1. User types `/` in the agent panel message editor
2. Autocomplete menu appears showing available slash commands (fuzzy-matched)
3. Both user-defined and ACP server commands appear, with different indicators:
   - User commands show "(user)" or "(user:namespace)" in description
   - Server commands show their server-provided description
4. User selects a command (e.g., `/review`)
5. User can provide arguments: `/review "security concerns"`
6. On submit, `$1` is replaced with "security concerns" and the expanded text is sent

### Argument Substitution

- `$ARGUMENTS` - All arguments as a single string (preserves original input)
- `$1`, `$2`, etc. - Positional placeholders
- Arguments support quoted strings: `/command "multi word arg" simple_arg`
- Unquoted arguments are space-separated
- Missing arguments result in an error (command is not sent)
- Escape sequences: `\$1` produces literal `$1`, `\"` produces literal `"`, `\n` produces newline

### Namespacing

Subdirectories create namespaces that appear in the command description:
- `commands/review.md` → `/review` with description "(user)"
- `commands/frontend/component.md` → `/component` with description "(user:frontend)"
- `commands/tools/git/commit.md` → `/commit` with description "(user:tools/git)"

## Decisions

| Question | Decision |
|----------|----------|
| Command location | `config_dir()/commands/` (uses Zed's paths helper) |
| File format | Markdown files (`.md` extension) |
| Missing arguments | Show error and don't send |
| ACP vs user command conflicts | Show both with different indicators |
| Duplicate command names | Error shown to user (no silent precedence) |
| Quoted arguments | Yes, support quoted multi-word arguments |
| $ARGUMENTS placeholder | Captures all arguments as-is |
| Escape sequences | Use backslash: `\$1`, `\"`, `\n` |
| @ mentions in templates | Treated as literal text |
| Feature flag | `user-slash-commands` |
| Frontmatter support | Not implemented (out of scope) |

## Implementation

### Files Modified/Created

#### `crates/feature_flags/src/flags.rs`
```rust
pub struct UserSlashCommandsFeatureFlag;

impl FeatureFlag for UserSlashCommandsFeatureFlag {
    const NAME: &'static str = "user-slash-commands";
}
```

#### `crates/agent_ui/src/user_slash_command.rs` (NEW)
Main module containing:
- `UserSlashCommand` struct - represents a loaded command with name, template, namespace, and path
- `commands_dir()` - returns `config_dir()/commands/`
- `load_user_commands()` - scans directory and loads all `.md` files
- `load_commands_from_dir()` - recursive directory traversal
- `load_command_file()` - loads single command file
- `commands_to_map()` - converts Vec to HashMap for lookup
- `try_parse_user_command()` - parses `/command args` syntax
- `parse_arguments()` - handles quoted/unquoted arguments
- `has_placeholders()` - checks for $1, $2, or $ARGUMENTS
- `count_positional_placeholders()` - counts highest $N
- `validate_arguments()` - ensures arg count matches template
- `expand_template()` - performs substitution
- `expand_user_slash_command()` - combines validation and expansion
- `try_expand_from_commands()` - high-level expansion function
- `has_command()` - checks if command exists

#### `crates/agent_ui/src/completion_provider.rs`
- Added `CommandSource` enum: `Server` vs `UserDefined { template }`
- Modified `search_slash_commands()` to load file-based commands
- User commands appear with "(user)" or "(user:namespace)" description

#### `crates/agent_ui/src/acp/message_editor.rs`
- Modified `validate_slash_commands()` to skip validation for user commands
- Modified `contents()` to expand user commands before sending

### Test Coverage (49 tests)

#### Argument Parsing
- Simple unquoted args
- Quoted args with spaces
- Mixed quoted/unquoted
- Escaped quotes within quotes
- Escaped backslash
- Empty quoted string
- Unclosed quote error
- Quote in middle of word error
- Unknown escape sequence error
- Newline escape in quotes

#### Template Expansion
- Basic $1 substitution
- Multiple placeholders $1, $2
- Repeated placeholder $1 $1
- Out of order $2 then $1
- Newline escape \n
- Dollar escape \$1
- Quote escape \"
- Backslash escape \\
- $ARGUMENTS placeholder
- $ARGUMENTS with positional
- $ARGUMENTS empty
- $ARGUMENTS preserves quotes

#### Validation
- Exact match count
- Missing arguments error
- Extra arguments error
- No placeholders, no args OK
- No placeholders with args error
- Empty template error
- $ARGUMENTS accepts any count
- Mixed $ARGUMENTS and positional

#### File Loading
- Load single command file
- Load with namespace (subdirectory)
- Load nested namespace (tools/git)
- Empty file ignored
- Load multiple files from directory
- Nonexistent directory returns empty
- commands_to_map conversion

#### Integration
- try_expand_from_commands with various inputs
- has_command lookup
- UserSlashCommand.description()
- UserSlashCommand.requires_arguments()

## Error Messages

- Missing argument: "The /review command requires 2 arguments, but only 1 was provided"
- Extra argument: "The /review command accepts 1 argument, but 2 were provided"
- Unclosed quote: "Unclosed quote in command arguments"
- Quote in middle: "Quote in middle of unquoted argument"
- Unknown escape: "Unknown escape sequence: \x"
- Empty template: "Template cannot be empty"
- Invalid placeholder: "Placeholder $0 is invalid; placeholders start at $1"

## Edge Cases

| Case | Behavior |
|------|----------|
| Empty template | Error: "Template cannot be empty" |
| Template with no placeholders | Works with zero arguments |
| Extra arguments beyond placeholders | Error with count mismatch |
| $ARGUMENTS only | Accepts any number of arguments (including zero) |
| Mixed $ARGUMENTS and $1 | Must have at least the positional args |
| Nested quotes | Not supported (error) |
| @ mention in template | Treated as literal text |
| Unknown escape `\x` | Error: "Unknown escape sequence" |
| Empty file | Ignored (not loaded as command) |
| Non-.md files | Ignored |
| Symlinked directories | Followed (can symlink ~/.claude/commands/) |
| Duplicate command names | Error: "Command 'X' is ambiguous: also defined at Y" |
