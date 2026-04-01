# Zed for iPad

My dream is to be able to write code from anywhere on a device as portable as a tablet. Finally, it's time to do serious work on an iPad.

## What this is

A port of Zed to iPadOS as a remote thin client. The iPad renders UI and handles input; all compute (LSP, git, file I/O, terminals, extensions) runs on a remote host via `zed --headless` over SSH. This builds on the existing remote development protocol — the iPad is a new platform front-end for the same `crates/remote/` infrastructure desktop Zed already uses.

## What's working

- **GPUI iOS platform layer** (`crates/gpui_ios/`, ~6K lines) — Metal renderer, CoreText text system, GCD dispatcher, CADisplayLink vsync, hardware keyboard, software keyboard, touch input, trackpad scroll/hover, UIPointerInteraction with custom cursor styles
- **Embedded SSH transport** (`crates/remote/src/transport/russh_ssh.rs`) — pure-Rust SSH via russh, replacing the system `ssh` binary that iOS doesn't have. Key auth, password auth, channel multiplexing, terminal PTY channels, and no-PTY command channels for agents
- **Connection landing screen** — saved hosts, auto-reconnect, workspace switcher, session persistence across backgrounding
- **Full editor** with LSP completions, diagnostics, git decorations — all proxied from the remote host
- **Terminal panel** — SSH-backed terminals with PTY over russh channels
- **Agent panel with external agents** — Claude Code runs on the remote host via ACP over SSH command channels. Keychain unlock flow for macOS remotes (user enters macOS password in an SSH terminal to unlock credentials)
- **Edit prediction** infrastructure (needs Zed Cloud auth to test end-to-end)
- **Debug panel**, settings profiles, workspace state persistence

101 commits across the `ipad` branch, plus the existing Zed codebase unchanged.

## Architecture

```
iPad (UI Shell)                         Remote Host
┌─────────────────────────────┐        ┌────────────────────────────┐
│ gpui_ios (Metal + CoreText) │        │ zed --headless             │
│ zed_ios (app init + FFI)    │◄──────►│ LSP, git, terminals, DAP   │
│ Workspace, Editor, Panels   │  SSH   │ Extensions, file I/O       │
│ Agent panel (LLM via HTTPS) │ russh  │ Agent tool execution       │
│ Keychain SSH auth           │        │ Agent binary hosting       │
└─────────────────────────────┘        └────────────────────────────┘
```

Two new crates: `gpui_ios` (iOS platform trait implementation) and `zed_ios` (staticlib entry point). A thin Swift host app in `ios/` bootstraps UIKit and calls into Rust via C FFI.

## What I'm looking for

Feedback on whether this is interesting to the Zed team. I understand this is a large fork and I want to be upfront: given the scope (new platform layer, SSH transport rewrite, agent proxy, ~10K lines of new code), I hope it's understandable why I chose to hack on it directly rather than go through the feature proposal process first. This has been a super fun project and I'd love for it to find a home upstream if there's interest.

## Fun fact

The [crate rename commit](https://github.com/dcow/zed/commit/197283beca) (`zed-ios` → `zed_ios`) was done entirely on a live instance of Zed for iPad, connected to the development Mac via SSH — the port editing itself.

## Docs

- `docs/ios-port-plan.md` — full architecture and phase plan
- `docs/ios-checklist.md` — working checklist of what's done and what's next
- `crates/zed_ios/CLAUDE.md` — crate-level docs
- `ios/README.md` — Xcode project and Swift host docs

Release Notes:

- N/A
