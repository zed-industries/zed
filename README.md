# Zed (ChxisB fork)

A self-learning, agentic fork of [Zed](https://zed.dev) — a high-performance, multiplayer code editor from the creators of [Atom](https://github.com/atom/atom) and [Tree-sitter](https://github.com/tree-sitter/tree-sitter).

## ✨ What's different

This fork extends Zed's native AI agent with persistent memory, skill curation, multi-agent routing, and autonomous task scheduling — turning the editor into a continuously improving development assistant.

| Feature | Description |
|---------|-------------|
| **Persistent memory** | `memory_write`/`memory_search` tools store facts across sessions in `~/.zed/memory.jsonl` |
| **Skill curator** | Watches completed threads, extracts repeating patterns, auto-generates `SKILL.md` files |
| **Multi-agent router** | 7 role-based profiles (Edit, Research, Terminal, Planning, Vision, Review, General) each with its own model |
| **Cron scheduler** | Background agent tasks on a timer — `cron_add`/`cron_list`/`cron_remove` tools + settings UI |
| **Webhook triggers** | Event-driven agent activation on file changes, HTTP requests, or git hooks |
| **User feedback → refinement** | Thumbs up/down on edits feeds back into the curator to strengthen or remove auto-skills |
| **Auto AGENTS.md** | AGENTS.md is auto-created when opening a new project with no existing rules file |

## Install

**macOS / Linux** (one command):
```bash
curl -fsSL https://raw.githubusercontent.com/ChxisB/zed/main/script/install.sh | bash
```

**Windows** (PowerShell):
```powershell
irm https://raw.githubusercontent.com/ChxisB/zed/main/script/install.ps1 | iex
```

Or download from [GitHub Releases](https://github.com/ChxisB/zed/releases).

To update, re-run the same command.

## Building from source

Requires Rust 1.81+ and Node.js 20+:

```bash
git clone https://github.com/ChxisB/zed.git
cd zed
cargo build --release
./target/release/zed
```

## Usage

```bash
zed                    # Open the editor
zed --help             # CLI options
```

Once inside, open the agent panel (✨ icon in status bar) and try:

- *"Remember that I prefer tabs over spaces"* → saves to persistent memory
- *"Check for npm vulnerabilities every morning at 9am"* → creates a cron job
- *"Watch for changes to *.rs files and run cargo check"* → creates a webhook trigger
- *"Plan the architecture for a microservice"* → routed to Planning agent (Claude)
- *"Review this code"* → routed to Review agent

## Agent Setup

**AI Settings → Agent Setup** configures which model each role uses:

| Role | Default Model | Tools |
|------|--------------|-------|
| Edit | anthropic/claude-sonnet-4 | All tools |
| Research | google/gemini-2.0-flash | search_web, fetch, read, grep |
| Terminal | openrouter/deepseek-chat | terminal, read_file |
| Planning | anthropic/claude-sonnet-4 | All tools, spawn_agent |
| Vision | google/gemini-2.0-flash | read_file, fetch |
| Review | anthropic/claude-sonnet-4 | read_file, grep, diagnostics |
| General | (inherited) | All tools |

## Licensing

Licensed under GPL-3.0-or-later, same as upstream Zed. See [LICENSE](./LICENSE) for details.
