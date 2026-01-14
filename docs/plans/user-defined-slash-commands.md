# User-Defined Slash Commands

## Status: ✅ COMPLETE

## COMPLETED WORK

### Async File Operations ✅ COMPLETE
- All file operations now use the `Fs` trait for async I/O
- `load_all_commands_async()` is the primary loading function
- Tests use `FakeFs` instead of real filesystem

### SlashCommandRegistry Entity & Caching ✅ COMPLETE
- `SlashCommandRegistry` struct caches commands in a `HashMap`
- Watches for file changes in commands directories
- Emits `SlashCommandRegistryEvent::CommandsChanged` when commands reload
- Integrated into `AcpThreadView` - each thread view has its own registry
- `MessageEditor.contents()` uses cached commands from registry (no per-request loading)
- Completion provider still loads async (acceptable since it's once per search, not per keystroke)

### Symlink Handling ✅ COMPLETE
Tests added to verify symlink behavior:
- `test_load_commands_from_symlinked_directory` - symlinked directories
- `test_load_commands_from_symlinked_file` - symlinked individual files
- `test_load_commands_claude_symlink_pattern` - common ~/.claude/commands/ symlink pattern

### Permission/Error Handling ✅ COMPLETE
Tests added to verify error handling:
- `test_load_commands_continues_after_single_file_error` - one bad file doesn't stop others
- `test_load_commands_reports_directory_read_errors` - directory errors are reported
- `test_command_load_error_includes_path_info` - errors include path information
- `test_load_all_commands_aggregates_errors` - multiple duplicate errors aggregated
- `test_empty_commands_directory_no_errors` - empty dirs don't cause errors
- `test_mixed_valid_and_empty_files` - empty files ignored, valid files loaded

### Error Handling UI ✅ COMPLETE
Implemented in `crates/agent_ui/src/acp/thread_view.rs`:
- Added `command_load_errors: Vec<CommandLoadError>` and `command_load_errors_dismissed: bool` fields
- Commands are loaded asynchronously on thread view initialization
- `render_command_load_errors()` displays a dismissable `Callout` with severity `Warning`
- Single error shows "Failed to load slash command", multiple errors shows "Failed to load N slash commands"

### Ambiguous Command Detection ✅ COMPLETE
If multiple commands have the same name (e.g., same command in two different project worktrees, or a project command and user command with the same name), an error is reported. There is no silent precedence.

### Test Coverage ✅ COMPLETE (52 tests)

**Parsing Tests:**
- `test_try_parse_user_command` - command parsing
- `test_parse_arguments_*` - argument parsing edge cases
- `test_count_positional_placeholders` - placeholder counting
- `test_has_placeholders` - placeholder detection

**Template Expansion Tests:**
- `test_expand_template_*` - substitution, escapes, $ARGUMENTS

**Validation Tests:**
- `test_validate_arguments_*` - argument count validation

**Edge Case Tests:**
- `test_unicode_command_names` - unicode in command names
- `test_unicode_in_arguments` - unicode in arguments
- `test_unicode_in_template` - unicode in templates
- `test_very_long_template` - large templates (100k chars)
- `test_many_placeholders` - templates with 10 placeholders
- `test_placeholder_zero_is_invalid` - `$0` errors
- `test_dollar_sign_without_number` - bare `$` preservation
- `test_consecutive_whitespace_in_arguments` - whitespace handling
- `test_empty_input` - empty/whitespace input
- `test_command_description_formats` - all description formats

**Async File Loading Tests (using FakeFs):**
- `test_load_commands_from_empty_dir`
- `test_load_commands_from_nonexistent_dir`
- `test_load_single_command`
- `test_load_commands_with_namespace`
- `test_load_commands_nested_namespace`
- `test_load_commands_empty_file_ignored`
- `test_load_commands_non_md_files_ignored`
- `test_load_project_commands`
- `test_load_all_commands_no_duplicates`
- `test_load_all_commands_duplicate_error`
- `test_registry_loads_commands`
- `test_registry_updates_worktree_roots`

**Symlink Tests:**
- `test_load_commands_from_symlinked_directory`
- `test_load_commands_from_symlinked_file`
- `test_load_commands_claude_symlink_pattern`

**Error Handling Tests:**
- `test_load_commands_continues_after_single_file_error`
- `test_load_commands_reports_directory_read_errors`
- `test_command_load_error_includes_path_info`
- `test_load_all_commands_aggregates_errors`
- `test_empty_commands_directory_no_errors`
- `test_mixed_valid_and_empty_files`

**Total: 61 tests**

---

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
    └── component.md    # Creates /frontend:component command
```

Project-specific commands go in `.zed/commands/`:
```
my-project/.zed/commands/
├── build.md            # Creates /build command with "(project)" description
└── deploy.md           # Creates /deploy command
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
   - Project commands show "(project)" or "(project:namespace)" in description
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
- `commands/frontend/component.md` → `/frontend:component` with description "(user:frontend)"
- `commands/tools/git/commit.md` → `/tools:git:commit` with description "(user:tools/git)"

## Decisions

| Question | Decision |
|----------|----------|
| Command location | `config_dir()/commands/` for user, `.zed/commands/` for project |
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

#### `crates/agent_ui/src/user_slash_command.rs`
Main module containing:
- `SlashCommandRegistry` - Entity that caches commands and watches for changes
- `UserSlashCommand` struct - represents a loaded command
- `CommandScope` enum - `Project` or `User`
- `CommandLoadResult` - commands and errors from loading
- `load_all_commands_async()` - async loading with `Fs` trait
- Parsing, validation, and expansion functions

#### `crates/agent_ui/src/completion_provider.rs`
- Added `CommandSource` enum: `Server` vs `UserDefined { template }`
- Modified `search_slash_commands()` to load file-based commands asynchronously

#### `crates/agent_ui/src/acp/message_editor.rs`
- Modified `validate_slash_commands()` to skip validation for user commands
- Modified `contents()` to expand user commands asynchronously before sending

#### `crates/agent_ui/src/acp/thread_view.rs`
- Added error display UI for command loading failures
- Spawns async task to load commands on initialization

## Error Messages

- Missing argument: "The /review command requires 2 arguments, but only 1 was provided"
- Extra argument: "The /review command accepts 1 argument, but 2 were provided"
- Unclosed quote: "Unclosed quote in command arguments"
- Quote in middle: "Quote in middle of unquoted argument"
- Unknown escape: "Unknown escape sequence: \x"
- Empty template: "Template cannot be empty"
- Invalid placeholder: "Placeholder $0 is invalid; placeholders start at $1"
- Duplicate command: "Command 'X' is ambiguous: also defined at Y"

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
| Unicode in command names | Supported |
| Very long templates | Supported (tested with 100k chars) |