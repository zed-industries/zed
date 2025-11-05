# Enhanced Context Commands Extension for Zed

Adds rich context gathering slash commands to Zed's AI assistant, inspired by NeuroNexus IDE's @-mention system.

**Developed by:** cloudraLabs

## Features

This extension provides five slash commands for adding specific context to AI conversations:

### `/context-file <path>`

Include a specific file's complete content in the conversation context.

```
/context-file src/main.rs
/context-file components/Header.tsx
```

**What it does:**
- Reads the entire file content
- Formats it for AI consumption with proper code fencing
- Shows file path and language
- Adds to conversation context

**Use case:** When you need the AI to reference a specific file's implementation.

### `/context-folder <path>`

Include all files in a folder recursively.

```
/context-folder src/components
/context-folder lib/utils
```

**What it does:**
- Recursively scans the folder
- Includes all files with their contents
- Shows file tree structure
- Displays line counts per file
- Filters out common ignore patterns (.git, node_modules, etc.)

**Use case:** When refactoring an entire module or understanding a component structure.

### `/context-symbol <name>`

Include a specific symbol's definition (function, class, interface, etc.).

```
/context-symbol UserAuthentication
/context-symbol calculateTotal
/context-symbol IConfigOptions
```

**What it does:**
- Searches codebase for the symbol
- Finds the definition location
- Includes the complete definition with context
- Shows file path and line number

**Use case:** When discussing or modifying a specific function/class without including entire files.

### `/context-terminal [lines]`

Include recent terminal output in the conversation.

```
/context-terminal
/context-terminal 100
```

**What it does:**
- Captures recent terminal output (default: 50 lines)
- Includes command prompts and results
- Formats for readability
- Highlights errors if present

**Use case:** Debugging compilation errors, test failures, or command output.

### `/context-git [status|diff|log]`

Include git repository information.

```
/context-git status
/context-git diff
/context-git log
```

**Subcommands:**
- `status` - Show modified/staged/untracked files (default)
- `diff` - Show detailed diff of changes
- `log` - Show recent commit history

**What it does:**
- Runs git commands in the workspace
- Formats output for AI consumption
- Includes file paths and change summaries

**Use case:** Understanding what changed, reviewing commits, or planning changes based on current state.

## Usage Examples

### Debugging a Build Error

```
You: The build is failing with a type error

/context-terminal

You: Can you help fix this error?
```

### Refactoring Multiple Components

```
You: I need to refactor the authentication flow

/context-folder src/auth
/context-git status

You: How should I restructure these files?
```

### Understanding a Function

```
You: How does the user validation work?

/context-symbol validateUser
/context-file tests/auth.test.ts

You: Can you explain the validation logic?
```

### Code Review Preparation

```
You: Help me review my changes

/context-git diff
/context-git status

You: What should I check before committing?
```

## Comparison with Built-in Commands

Zed already has some context commands. This extension complements them:

| Built-in | This Extension | Difference |
|----------|----------------|------------|
| `/file` | `/context-file` | Similar functionality |
| `/tab` | - | Current tab only |
| `/docs` | - | External documentation |
| - | `/context-folder` | **NEW:** Entire folders |
| - | `/context-symbol` | **NEW:** Symbol lookup |
| - | `/context-terminal` | **NEW:** Terminal output |
| - | `/context-git` | **NEW:** Git integration |

## Benefits

1. **Fine-Grained Control** - Include exactly what you need
2. **Reduced Token Usage** - Don't include entire files when you only need a symbol
3. **Better AI Responses** - More relevant context = better suggestions
4. **Debugging Help** - Terminal and git integration for error diagnosis
5. **Workflow Integration** - Natural fit with development workflow

## Integration Points

This extension uses Zed's:
- Worktree API for file access
- Project API for symbol search
- Terminal integration for output capture
- Git integration for repository information

## Installation

This extension is built into Zed. Commands are available immediately in the AI assistant.

## Development

Built with:
- Rust (edition 2021)
- zed_extension_api 0.2.0
- Compiled to WASM for security

Source: `extensions/context-commands/src/lib.rs`

## Future Enhancements

Potential additions:
- `/context-codebase` - Semantic search across entire codebase
- `/context-dependencies` - Show package dependencies
- `/context-tests` - Include related test files
- Smart context detection - Automatic context based on query

## Tips

**Combine commands for rich context:**
```
/context-symbol authenticateUser
/context-file src/auth/auth.service.ts
/context-terminal 50
/context-git diff
```

**Use sparingly to save tokens:**
- Include only what's relevant
- Use `/context-symbol` instead of `/context-file` when possible
- Specify line limits for terminal output

**Best practices:**
- Add context before asking questions
- Include error messages via `/context-terminal`
- Show git status before planning changes
- Use folders for architectural questions

## License

Same as Zed (Apache 2.0 / GPL)
