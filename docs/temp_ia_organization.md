# Proposed Zed Documentation Information Architecture

This document outlines a proposed restructuring of Zed's documentation to improve discoverability, separate task-oriented content from reference material, and better serve users with different goals (new users, migrating users, power users).

---

## Proposed IA Structure

```markdown
# Get Started

- Welcome # Hub page with job-to-be-done cards
- Installation
- Your First 10 Minutes in Zed # True quickstart: open project → write code → use AI → run
- Key Concepts # Mental models: workspaces, buffers, panels, command palette

# Quickstarts # Task-focused, 5-10 min each

- Set Up AI in Zed
- Work on a Remote Server
- Configure Your Language
- Customize Zed to Feel Like Home

# Coming From... # Migration guides

- VS Code
- Vim / Neovim
- JetBrains
- Cursor

# AI # Key differentiator - prominent placement

- Overview
- Agent Panel
  - Tools
  - External Agents
- Inline Assistant
- Edit Prediction
- Text Threads
- Rules
- Model Context Protocol
- Configuration
  - LLM Providers
  - Agent Settings
- Subscription
  - Billing
  - Models
  - Plans and Usage

# Working with Code # Daily workflow, organized by job

- Overview # Walkthrough: open → edit → navigate → run → commit
- Editing Code
  - Code Completions
  - Snippets
  - Formatting & Linting
  - Diagnostics & Quick Fixes
  - Multibuffers
- Finding & Navigating
  - Command Palette
  - File Finder & Project Search
  - Go to Definition & Symbol Search
  - Outline Panel
- Running & Testing
  - Tasks
  - Terminal
  - Debugger
  - REPL
- Git

# Collaboration

- Overview
- Channels
- Private Calls

# Remote Development

- Overview
- SSH Connections
- Dev Containers

# Languages

- Overview # Hub with capability tags (LSP, Debug, Format, etc.)
- Configuring Languages
  - Toolchains
- Popular Languages # Curated, workflow-focused
  - Python
  - JavaScript / TypeScript
  - Rust
  - Go
  - C / C++
- All Languages # Searchable/filterable reference

# Customization # Personalization, organized by goal

- Overview # "Customize in 5 minutes" + how settings work
- Appearance
  - Themes
  - Icon Themes
  - Fonts & Visual Tweaks
- Keybindings
  - Custom Key Bindings
  - Vim Mode
  - Helix Mode
- Snippets

# Extensions

- Overview
- Installing Extensions
- Developing Extensions
- Extension Capabilities
- Extension Types
  - Language Extensions
  - Theme Extensions
  - Icon Theme Extensions
  - Debugger Extensions
  - Slash Command Extensions
  - Agent Server Extensions
  - MCP Server Extensions

# Reference # Lookup-only, not for reading

- All Settings
- All Actions
- Default Key Bindings
- CLI Reference
- Environment Variables
- Glob Patterns

# Platform Support

- macOS
- Windows
- Linux

# Account & Privacy

- Sign In
- Privacy and Security
- Telemetry

# Troubleshooting

- Common Issues
- Performance
- Update
- Uninstall

# Developing Zed

- Contributing
- Building from Source
  - macOS
  - Linux
  - Windows
  - FreeBSD
- Local Collaboration
- Using Debuggers
- Glossary
- Release Notes
```

---

## Deep Dive: Working with Code

This section replaces the current "Using Zed" catch-all. Instead of listing features, it's organized around **what users are trying to accomplish** in their daily workflow.

### Design Principles

1. **Overview as workflow guide**: The overview page walks through a typical session (open project → write code → find things → run/test → commit), linking to detailed pages
2. **Group by job, not feature**: "Finding & Navigating" groups file finder, symbol search, go-to-definition, and outline because they all serve the job of "I need to find something"
3. **Progressive disclosure**: Start with the common case, link to advanced topics

### Proposed Structure

**Working with Code**

```
Overview
├── "A day in Zed" walkthrough
├── Links to each sub-section
└── Quick tips for common tasks

Editing Code
├── Code Completions (LSP, AI-powered, snippets integration)
├── Snippets (creating, using, language-specific)
├── Formatting & Linting (format on save, linter integration)
├── Diagnostics & Quick Fixes (error navigation, code actions)
└── Multibuffers (what they are, when to use them)

Finding & Navigating
├── Command Palette (the hub for everything)
├── File Finder & Project Search (fuzzy find, ripgrep, filters)
├── Go to Definition & Symbol Search (LSP navigation, workspace symbols)
└── Outline Panel (file structure, breadcrumbs)

Running & Testing
├── Tasks (defining, running, task templates)
├── Terminal (integrated terminal, shell integration)
├── Debugger (breakpoints, stepping, variables, DAP)
└── REPL (interactive development, Jupyter-style)

Git
├── Staging & committing
├── Branches
├── Diff view
├── Blame & history
└── (future: GitHub integration)

```

### What This Replaces

| Current "Using Zed" Item | New Location                             |
| ------------------------ | ---------------------------------------- |
| Multibuffers             | Working with Code → Editing Code         |
| Command Palette          | Working with Code → Finding & Navigating |
| Command-line Interface   | Reference → CLI Reference                |
| Outline Panel            | Working with Code → Finding & Navigating |
| Code Completions         | Working with Code → Editing Code         |
| Collaboration            | Collaboration (top-level section)        |
| Git                      | Working with Code → Git                  |
| Debugger                 | Working with Code → Running & Testing    |
| Diagnostics              | Working with Code → Editing Code         |
| Tasks                    | Working with Code → Running & Testing    |
| Remote Development       | Remote Development (top-level section)   |
| Dev Containers           | Remote Development                       |
| Environment Variables    | Reference                                |
| REPL                     | Working with Code → Running & Testing    |

---

## Deep Dive: Customization

This section replaces "Configuration" and reframes it around **user goals** rather than settings categories.

### Design Principles

1. **Start with outcomes**: Users don't want to "configure settings" — they want Zed to look and feel a certain way
2. **Quick wins first**: Overview page shows how to customize the essentials in 5 minutes
3. **Explain the system**: Help users understand how settings work (JSON, project vs global, settings UI, deep links)
4. **Separate appearance from behavior**: Visual customization is different from keybinding customization

### Proposed Structure

**Customization**

```
Overview
├── "Customize Zed in 5 minutes" quick guide
├── How settings work (JSON, project vs global, settings UI)
├── Using settings deep links
└── Links to detailed sections

Appearance
├── Themes (installing, switching, creating)
├── Icon Themes
├── Fonts & Visual Tweaks (UI density, status bar, tab bar, etc.)

Keybindings
├── Custom Key Bindings (understanding the keymap, common customizations)
├── Vim Mode (setup, differences from native vim, customization)
├── Helix Mode (setup, key differences)

Snippets
├── Using snippets
├── Creating custom snippets
├── Language-specific snippets

```

### What This Replaces

| Current "Configuration" Item | New Location                      |
| ---------------------------- | --------------------------------- |
| Configuring Zed              | Reference → All Settings          |
| Configuring Languages        | Languages → Configuring Languages |
| Key bindings                 | Customization→ Keybindings        |
| All Actions                  | Reference → All Actions           |
| Snippets                     | Customization→ Snippets           |
| Themes                       | Customization→ Appearance         |
| Icon Themes                  | Customization→ Appearance         |
| Visual Customization         | Customization→ Appearance         |
| Vim Mode                     | Customization→ Keybindings        |
| Helix Mode                   | Customization→ Keybindings        |

### Key Insight: Settings Reference vs. Customization Guide

The current `configuring-zed.md` is a **reference** (lookup all 200+ settings). That belongs in the Reference section.

"Making Zed Yours" is a **guide** that helps users accomplish goals:

- "I want Zed to look like my old editor" → Themes, fonts
- "I want to use my vim muscle memory" → Vim Mode
- "I want to change what ⌘+P does" → Custom Key Bindings

The guide pages should link to the reference when users need to look up specific settings.

---

## Summary of Changes

| Current                               | Proposed                                                       | Rationale                                                                |
| ------------------------------------- | -------------------------------------------------------------- | ------------------------------------------------------------------------ |
| Getting Started (5 items)             | Get Started (4) + Quickstarts (4) + Coming From... (4)         | Split welcome/install from task-focused quickstarts and migration guides |
| Configuration (9 items, front-loaded) | Customization(goal-oriented) + Reference                       | Users want outcomes, not settings; reference is for lookup               |
| Using Zed (14 items, catch-all)       | Working with Code (job-organized) + Collaboration + Remote Dev | Group by workflow: editing, navigating, running, git                     |
| AI (section 5)                        | AI (section 4)                                                 | Promote key differentiator                                               |
| Language Support (60+ flat list)      | Languages with Popular grouping + filterable All Languages     | Reduce cognitive load, highlight common languages                        |
| No Reference section                  | Reference (6 items)                                            | Explicitly label lookup-only content                                     |
| Troubleshooting in Getting Started    | Troubleshooting as dedicated section                           | Easier to find when users need help                                      |

---

## Content Gaps: New Pages Needed

### Get Started & Quickstarts

- `welcome.md` - Hub page with job-to-be-done cards
- `first-10-minutes.md` - True quickstart walkthrough
- `concepts.md` - Mental models for Zed
- `quickstarts/ai-setup.md`
- `quickstarts/remote-dev.md`
- `quickstarts/language-setup.md`
- `quickstarts/customize.md`

### Migration Guides

- `migration/vscode.md` ✅
- `migration/vim.md`
- `migration/jetbrains.md`
- `migration/cursor.md`

### Working with Code

- `working-with-code/overview.md` - "A day in Zed" walkthrough
- `working-with-code/editing/formatting-linting.md`
- `working-with-code/navigating/file-finder.md`
- `working-with-code/navigating/symbols.md`

### Customization

- `customization/overview.md` - "Customize in 5 minutes" + how settings work
- `customization/appearance/fonts-visual.md`

### Platform Support

- `platforms/macos.md` (currently missing)

### Reference

- `reference/default-keybindings.md`

---

## Pages to Rework

| Page                 | Changes Needed                                             |
| -------------------- | ---------------------------------------------------------- |
| `configuring-zed.md` | Relabel as "All Settings" reference, add on-page filtering |
| `all-actions.md`     | Add filtering by category (Agent, Editor, Git, etc.)       |
| `languages.md`       | Transform into filterable hub with capability tags         |
| `quick-start.md`     | Rewrite as true 5-10 minute walkthrough                    |

---

## Doc Types

Each page should be labeled with a type for clarity:

| Type           | Purpose                      | Example            |
| -------------- | ---------------------------- | ------------------ |
| **Quickstart** | 5-10 min, end-to-end task    | "Set Up AI in Zed" |
| **Guide**      | Deeper task-oriented content | "Vim Mode"         |
| **Concept**    | Explanatory, mental models   | "Key Concepts"     |
| **Reference**  | Lookup-only, tables/lists    | "All Settings"     |
