# Agent Settings UI Migration Plan

## Goal

Move agent configuration out of the agent panel and into the Settings UI,
under the AI page. Profiles, providers, MCP servers, and external agents
all belong in Settings. Rules get a link but stay in their own UI.

## Architecture

Two levels of interaction, no navigation for the common case:

1. **Inline expansion** (expandable cards) for glance-and-tweak settings
2. **Sub-page navigation** for deep-dive settings (long tool checklists, regex patterns)

## AI Page Structure

```
AI Page
├── General (disable AI toggle)
├── LLM Providers [SubPageLink → expandable provider cards with config views]  ✅
├── MCP Servers [SubPageLink → server list with status/source/tools]
├── External Agents [SubPageLink → agent list]
├── Rules [ActionLink → opens rules library in workspace]  ✅
├── Agent Profiles [SubPageLink → expandable profile cards]
│   ├── Profile card (expand: model dropdown, "Configure Tools ▸", "Configure MCP Tools ▸")
│   ├── Builtin profiles are non-deletable, custom profiles have fork/delete
│   └── [+ Add Profile] button
│       ├── "Configure Tools" sub-sub-page (toggle checklist with search)
│       └── "Configure MCP Tools" sub-sub-page (grouped toggle checklist)
├── Agent Configuration (existing boolean/enum settings)
├── Context Servers (timeout setting)
└── Edit Predictions (existing provider setup + display settings)
```

## Implementation Status

### Done
- [x] Dependencies added to `settings_ui/Cargo.toml` (language_model, language_models, client)
- [x] `llm_provider_setup.rs` — expandable provider cards with cached AnyView config
- [x] Rules ActionLink dispatching OpenRulesLibrary to workspace
- [x] Page registrations in `pages.rs` and `page_data.rs`

### Next: Agent Profiles Sub-page
- [ ] `agent_profile_setup.rs` — expandable profile cards
- [ ] Each profile card shows: name, icon, current model summary, tool count
- [ ] Expanded state: model picker/dropdown, "Configure Tools ▸", "Configure MCP Tools ▸"
- [ ] Builtin profiles: fork only. Custom profiles: fork + delete
- [ ] "Add Profile" button (text input for name, optional base profile to fork from)
- [ ] Tool configuration sub-sub-page (toggle checklist, ~19 builtin tools)
- [ ] MCP tool configuration sub-sub-page (grouped by server, toggle checklist)

### Next: MCP Servers Sub-page
- [ ] `mcp_server_setup.rs` — per-server cards with rich detail
- [ ] Show status indicator (running/stopped/error/starting)
- [ ] Show source badge (extension vs custom)
- [ ] Show tool count when running
- [ ] Start/stop toggle per server
- [ ] Configure/uninstall actions per server
- [ ] "Install from Extensions" and "Add Custom Server" actions
- [ ] Needs to reconcile ContextServerDescriptorRegistry (global, config) with
      ContextServerStore (project-level, runtime state)

### Next: External Agents Sub-page
- [ ] `external_agent_setup.rs` — list agents from AgentServerStore
- [ ] Needs project-level store access from the settings window
- [ ] Show agent icon, display name, source for each agent
- [ ] "Add Agent" actions (install from registry, add custom)

### Later
- [ ] "Add Provider" action (currently lives in agent_ui as AddLlmProviderModal)
- [ ] Update agent panel menu: "Settings" opens Settings UI AI page
- [ ] Remove old `AgentConfiguration` view from agent panel
- [ ] Update ProfileSelector footer "Configure" to navigate to settings UI

## Design Rationale

- **Expandable cards** (not sub-page drill-down) for providers and profiles
  because each item has few settings. Avoids navigation maze.
- **Sub-pages** for tool checklists because 19+ items need search/scroll space.
- **Eager view creation** for provider configs to avoid "Loading credentials..." flash.
- **`in_json: false`** on sub-pages because provider config involves keychains,
  not just JSON fields.
