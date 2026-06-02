# claude_code_ide

Native [Claude Code](https://docs.claude.com/en/docs/claude-code) IDE integration
for Zed.

This crate lets the `claude` CLI connect to Zed the same way it connects to
VS Code, JetBrains and Neovim — over Claude Code's native WebSocket + MCP
"IDE integration" protocol. It is **not** ACP: Zed already exposes Claude Code
as an ACP agent in the agent panel; this is the complementary direction, where
`claude` running in Zed's integrated terminal drives the editor.

## What you get

- **Auto-connect** in Zed's integrated terminal — no `/ide` needed.
- `@`-mentions of your current selection (`getCurrentSelection`).
- Model-visible **diagnostics** (`getDiagnostics`).
- Open editors / open file / save / dirty checks
  (`getOpenEditors`, `openFile`, `saveDocument`, `checkDocumentDirty`).
- **Blocking accept/reject diffs** (`openDiff`): Claude's proposed change opens
  as a side-by-side diff tab, the view centers on the first change, and the CLI
  blocks until you click **Keep** (green) or **Reject** (red).

## How it works

1. Each workspace starts a per-window server: it binds `127.0.0.1:0`, then
   writes a lockfile to `~/.claude/ide/<port>.lock` (honoring
   `CLAUDE_CONFIG_DIR`) with `0600` perms inside a `0700` dir. The lockfile
   advertises `{pid, workspaceFolders, ideName: "Zed", transport: "ws",
   authToken}` so the CLI can discover the IDE.
2. The transport is a WebSocket authenticated by the
   `x-claude-code-ide-authorization` header (which must equal `authToken`),
   speaking JSON-RPC 2.0 / MCP (`initialize`, `tools/list`, `tools/call`),
   protocol version `2024-11-05`.
3. Auto-connect: the workspace publishes the server port onto `Project`, and
   `crates/project/src/terminals.rs` injects `CLAUDE_CODE_SSE_PORT` and
   `ENABLE_IDE_INTEGRATION=true` into the integrated terminal, so `claude`
   connects automatically. An external terminal can still attach via `/ide`,
   discovering Zed from the lockfile.

Entry point: `claude_code_ide::init(cx)`, called from `crates/zed/src/main.rs`.

## Tools

| Tool | Purpose |
| --- | --- |
| `getCurrentSelection` / `getLatestSelection` | Active editor selection (for `@`-mentions). |
| `getWorkspaceFolders` | Visible worktree roots. |
| `getOpenEditors` | Open buffers with uri/label/language/dirty state. |
| `getDiagnostics` | Language diagnostics, all buffers or one uri. |
| `openFile` | Open a path, optionally selecting a line range. |
| `saveDocument` / `checkDocumentDirty` | Save a buffer / query its dirty state. |
| `openDiff` | Blocking side-by-side diff with Keep/Reject. |

## Try it

```bash
cargo run --release -p zed   # Linux: run ./script/linux once for build deps
```

Open Zed's integrated terminal and run `claude` — it connects automatically.
Edit a file through Claude and a Keep/Reject diff opens in the editor.

## Limitations

- Keep/Reject hotkeys are intentionally omitted; use the notification buttons.

