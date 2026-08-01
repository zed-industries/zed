---
title: CLI Reference
description: "Reference for Zed's command-line interface (CLI), including opening files and directories, integrating with tools, and controlling Zed from scripts."
---

# CLI Reference

Use Zed's command-line interface (CLI) to open files and directories, integrate with other tools, and control Zed from scripts.

## Installation

**macOS:** Run the {#action cli::InstallCliBinary} command from the command palette ({#kb command_palette::Toggle}) to install the `zed` CLI to `/usr/local/bin/zed`.

**Linux:** The CLI is included with Zed packages. The binary name may vary by distribution (commonly `zed` or `zeditor`).

**Windows:** The CLI is included with Zed. Add Zed's installation directory to your PATH, or use the full path to `zed.exe`.

## Usage

```sh
zed [OPTIONS] [PATHS]...
```

## Opening Files and Directories

Open a file:

```sh
zed myfile.txt
```

Open a directory as a workspace:

```sh
zed ~/projects/myproject
```

Open multiple files or directories:

```sh
zed file1.txt file2.txt ~/projects/myproject
```

Open a file at a specific line and column:

```sh
zed myfile.txt:42        # Open at line 42
zed myfile.txt:42:10     # Open at line 42, column 10
```

## Options

### `-w`, `--wait`

Wait for all opened files to be closed before the CLI exits. When opening a directory, waits until the window is closed.

This is useful for integrating Zed with tools that expect an editor to block until editing is complete (e.g., `git commit`):

```sh
export EDITOR="zed --wait"
git commit  # Opens Zed and waits for you to close the commit message file
```

### `-n`, `--new`

Open paths in a new workspace window, even if the paths are already open in an existing window:

```sh
zed -n ~/projects/myproject
```

### `-a`, `--add`

Add paths to the currently focused workspace instead of opening a new window. When multiple workspace windows are open, files open in the focused window:

```sh
zed -a newfile.txt
```

### `-r`, `--reuse`

Reuse an existing window, replacing its current workspace with the new paths:

```sh
zed -r ~/projects/different-project
```

### `-e`, `--existing`

Open paths in an existing Zed window instead of creating a new one:

```sh
zed -e myfile.txt
```

By default (without `-n`, `-a`, `-r`, or `-e`), directories open in the current window's sidebar. You can change this default with the `cli_default_open_behavior` setting. See [Windows & Projects](../windows-and-projects.md) for more details.

### `--diff <OLD_PATH> <NEW_PATH>`

Open a diff view comparing two files. Can be specified multiple times:

```sh
zed --diff file1.txt file2.txt
zed --diff old.rs new.rs --diff old2.rs new2.rs
```

### `--foreground`

Run Zed in the foreground, keeping the terminal attached. Useful for debugging:

```sh
zed --foreground
```

### `--user-data-dir <DIR>`

Use a custom directory for all user data (database, extensions, logs) instead of the default location:

```sh
zed --user-data-dir ~/.zed-custom
```

Default locations:

- **macOS:** `~/Library/Application Support/Zed`
- **Linux:** `$XDG_DATA_HOME/zed` (typically `~/.local/share/zed`)
- **Windows:** `%LOCALAPPDATA%\Zed`

### `-v`, `--version`

Print Zed's version and exit:

```sh
zed --version
```

### `--completions <SHELL>`

Generate shell completions for the `zed` CLI:

#### Bash

Add to `~/.bashrc`:

```bash
eval "$(zed --completions bash)"
```

#### Elvish

Add to `~/.config/elvish/rc.elv`:

```elvish
set edit:completion:arg-completer[zed] = { |@args|
    eval (zed --completions elvish | slurp)
    $edit:completion:arg-completer[zed] $@args
}
```

#### Fish

Add to `~/.config/fish/config.fish`:

```fish
zed --completions fish | source
```

#### Nushell

Add to `~/.config/nushell/config.nu`:

```nu
mkdir ($nu.data-dir | path join "vendor/autoload")
^zed --completions nushell | save --force ($nu.data-dir | path join "vendor/autoload/zed.nu")
```

#### Powershell

Add to `$PROFILE`:

```powershell
(&zed --completions powershell) | Out-String | Invoke-Expression
```

#### Zsh

Add to `~/.zshrc`:

```zsh
eval "$(zed --completions zsh)"
```

### `--uninstall`

Uninstall Zed and remove all related files (macOS and Linux only):

```sh
zed --uninstall
```

### `--zed <PATH>`

Specify a custom path to the Zed application or binary:

```sh
zed --zed /path/to/Zed.app myfile.txt
```

## Sending Prompts to the Agent

The CLI can start an agent turn in a running Zed instance, which is useful for driving the agent from scripts, git hooks, or webhooks.

Send a prompt to the most recently updated thread for the current project:

```sh
zed --agent "Summarize the changes on this branch"
```

The Agent Panel does not need to be focused, and the prompt is never written into an editor you are typing in. If the agent is already generating, the prompt is added to that thread's message queue and sent when the current turn finishes, exactly as if you had typed it while the agent was working.

### `--agent`

Send a prompt. The prompt is taken from the remaining arguments, or from stdin when the prompt is `-`:

```sh
zed --agent "Fix the failing test in parser.rs"
curl -s https://example.com/task | zed --agent -
```

On success the target thread's id is printed to stdout, so scripts can capture it and keep talking to the same thread:

```sh
thread=$(zed --agent --agent-new "Start reviewing this PR")
zed --agent --agent-thread "$thread" "Now check the tests"
```

### `--agent-list`

List known threads, most recently updated first. Threads currently open in a window are marked `(open)`:

```sh
zed --agent-list
```

```text
567079b5-f231-4837-8283-23581f508113  just now  Fix failing parser test  (open)
8bb41db0-b3cd-4f45-b2eb-4dc49cc57602  2h ago  Add CLI reference docs
```

### `--agent-list-format <FORMAT>`

Either `text` (the default, shown above) or `json`. JSON emits an array — including when nothing matches — so scripts can pipe it straight into `jq` without parsing columns:

```sh
zed --agent-list --agent-list-format json
```

```json
[
  {
    "id": "567079b5-f231-4837-8283-23581f508113",
    "title": "Fix failing parser test",
    "updated_at": "2026-01-02T03:04:05+00:00",
    "interacted_at": "2026-01-02T03:03:11+00:00",
    "is_open": true,
    "paths": ["/Users/me/projects/myproject"]
  }
]
```

`updated_at` moves whenever the thread changes, including while the agent is working. `interacted_at` moves when you send, queue, retry, or regenerate a message in the Agent Panel, so it tells you when a person last engaged with the thread; prompts delivered by the CLI deliberately leave it alone. `paths` are the thread's worktree folders, which is what lets you match a thread to a specific git worktree:

```sh
# The most recently updated thread for the worktree checked out at $worktree
zed --agent-list --agent-list-format json --agent-project "$worktree" \
  | jq -r '.[0].id'
```

### `--agent-thread <ID>`

Target a specific thread. The id may be given in full or as a unique leading fragment, and hyphens are optional, so `567079b5-f231` and `567079b5f231` select the same thread:

```sh
zed --agent --agent-thread 567079b5 "Add a test for that"
```

If the fragment matches more than one thread, Zed reports the candidates instead of guessing.

### `--agent-new`

Always start a new thread rather than continuing an existing one:

```sh
zed --agent --agent-new "Investigate the flaky CI job"
```

### `--agent-project <DIR>`

Scope thread lookup and creation to a project. Defaults to the working directory, so running the CLI anywhere inside a project targets that project's threads:

```sh
zed --agent --agent-project ~/projects/myproject "What changed today?"
```

### `--agent-profile <PROFILE>`

Create the thread under a specific [agent profile](../ai/agent-profiles.md), which decides the tools the agent may use. This matters for unattended runs, where a narrower profile limits what a prompt can do:

```sh
zed --agent --agent-new --agent-profile ask "Summarize what changed on this branch"
```

An unknown profile is an error listing the configured ones, rather than a silent fallback to the default.

### `--agent-model <PROVIDER/MODEL>`

Create the thread with a specific model:

```sh
zed --agent --agent-new --agent-model anthropic/claude-sonnet-4-5 "Review this diff"
```

An unconfigured model is an error.

Both flags require `--agent-new`. A thread stores its profile and model, and changing the profile also switches the model and applies to any subagents it is running. They configure a thread you are creating rather than silently reconfiguring one you already have open. To run a prompt under a different profile, start a new thread for it.

### `--agent-wait`

Block until the agent finishes its turn:

```sh
zed --agent --agent-wait "Run the test suite and fix what breaks"
```

There is deliberately no timeout. If the agent asks for permission to run a tool, it waits for you to answer in the Agent Panel, the same as a prompt you typed yourself. See [Agent Settings](../ai/agent-settings.md) for how to configure tool permissions.

### `--agent-session <SESSION_ID>`

Target a thread by its Agent Client Protocol session id. This is mainly useful for tooling that already tracks ACP sessions:

```sh
zed --agent --agent-session 8bb41db0-b3cd-4f45-b2eb-4dc49cc57602 "Continue this review"
```

### Targeting a Git Worktree

Because threads remember the worktree folders they belong to, a hook that knows a branch can find the right thread by resolving the branch to its worktree first:

```sh
worktree=$(git -C "$repo" worktree list --porcelain \
  | awk -v branch="refs/heads/$branch" '/^worktree /{path=$2} $0=="branch "branch{print path}')

zed --agent --agent-project "$worktree" "CI failed on $branch: $details"
```

Pass `--agent-project` explicitly rather than relying on the working directory. A thread created in a linked worktree also records the main checkout, so a lookup made from the main checkout can match threads belonging to any of its linked worktrees, while a lookup made from the linked worktree matches only its own.

The worktree must already be open in Zed. If it isn't, the CLI reports that rather than falling back to another project, so a hook can open it first:

```sh
git -C "$repo" worktree add "$worktree" "$branch"
zed "$worktree"
zed --agent --agent-project "$worktree" --agent-new "Investigate the CI failure on $branch"
```

## Reading from Standard Input

Read content from stdin by passing `-` as the path:

```sh
echo "Hello, World!" | zed -
cat myfile.txt | zed -
ps aux | zed -
```

This creates a temporary file with the stdin content and opens it in Zed.

## URL Handling

The CLI can open `zed://`, `file://`, and `ssh://` URLs:

```sh
zed zed://settings
zed file:///Users/whatever/.zshrc
zed ssh://me@example.com/abs/path
zed ssh://me@example.com:/abs/path
zed ssh://me@example.com/~/project
zed ssh://me@example.com:~/project
```

## Using Zed as Your Default Editor

Set Zed as your default editor for Git and other tools:

```sh
export EDITOR="zed --wait"
export VISUAL="zed --wait"
```

Add these lines to your shell configuration file (e.g., `~/.bashrc`, `~/.zshrc`).

## macOS: Switching Release Channels

On macOS, you can launch a specific release channel by passing the channel name as the first argument:

```sh
zed --stable myfile.txt
zed --preview myfile.txt
zed --nightly myfile.txt
```

## WSL Integration (Windows)

On Windows, the CLI supports opening paths from WSL distributions. This is handled automatically when launching Zed from within WSL.

## Exit Codes

| Code | Meaning                           |
| ---- | --------------------------------- |
| `0`  | Success                           |
| `1`  | Error (details printed to stderr) |

When using `--wait`, the exit code reflects whether the files were saved before closing.
