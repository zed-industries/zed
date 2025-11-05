# Checkpoint Commands Extension for Zed

Adds checkpoint and rollback functionality to Zed's AI assistant, inspired by NeuroNexus IDE.

**Developed by:** cloudraLabs

## Features

This extension provides three slash commands for managing conversation checkpoints:

### `/checkpoint [description]`

Create a checkpoint to save the current state of all modified files.

```
/checkpoint Before refactoring authentication
```

**What it does:**
- Saves snapshots of all modified files at the current point
- Associates checkpoint with current message in thread
- Allows optional description for easy identification

### `/rollback <checkpoint-id>`

Rollback to a previous checkpoint, restoring all files to their state at that point.

```
/rollback 2
```

**What it does:**
- Restores all files to the specified checkpoint state
- Creates a safety checkpoint before rollback (so you can undo the rollback)
- Shows which files will be affected

### `/list-checkpoints`

List all available checkpoints in the current conversation thread.

```
/list-checkpoints
```

**Output:**
- Checkpoint number and ID
- Timestamp when created
- Type (user/agent/automatic)
- Number of files affected
- Description (if provided)
- Current position indicator

## Use Cases

### 1. Safe Experimentation
```
/checkpoint Before trying experimental approach
[AI makes changes]
[Not happy with results]
/rollback 1
```

### 2. Branching Conversations
```
/checkpoint Completed feature A
[Try approach 1]
/rollback 1
[Try approach 2 instead]
```

### 3. Tracking Progress
```
/list-checkpoints
[See all checkpoints]
/rollback 3
[Go back to specific point]
```

## Integration with Checkpoint System

This extension interfaces with Zed's core checkpoint system implemented in `crates/agent/src/checkpoint.rs`. The core system handles:
- File snapshot storage
- Content versioning
- Rollback mechanics
- History management

The extension provides user-friendly commands on top of this system.

## Installation

This extension is built into Zed. The commands are available immediately in any AI assistant thread.

## Development

Built with:
- Rust (edition 2021)
- zed_extension_api 0.2.0
- Compiled to WASM for sandboxed execution

Source code: `extensions/checkpoint-commands/src/lib.rs`

## License

Same as Zed (Apache 2.0 / GPL)
