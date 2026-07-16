# Phase 2: Autonomous Agent Features

Build order (highest impact first):

1. **Cron scheduler** — background tasks on a schedule
2. **Webhook triggers** — event-driven agent activation
3. **User feedback → skill refinement** — learn from thumbs
4. **Project-scoped memory** — separate namespaces
5. **Multi-step planning + checkpoint/restore** — plan→execute→rollback
6. **Agent-initiated tool creation** — agent writes MCP tools
7. **Agent-to-agent comms** — sub-agent message bus
8. **Self-hosted model auto-discovery** — detect ollama/lm-studio

## Architecture decisions

- All new modules live under `crates/agent/src/` following the curator/router pattern
- Persistent state uses JSON files in `~/.zed/` (matching the memory store)
- Cron scheduler uses a simple tokio interval loop, not a full cron daemon
- Webhooks use a simple HTTP listener on localhost
- Everything defaults to disabled — opt-in via settings.json
