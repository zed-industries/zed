# Convergio Studio Changelog

All notable Convergio-specific changes to this project will be documented in this file.

This project follows [Semantic Versioning](https://semver.org/).

## [0.1.0] - 2025-12-20

### Added

#### Agent System
- **54 Specialized AI Agents** organized in 14 categories (Leadership, Technology, Security, Product, etc.)
- **Ali - AI Chief of Staff** with persistent memory across all conversations
- **Convergio Panel** - dedicated dock panel for agent interactions
- **Agent Packs** - presets for different workflows:
  - Enterprise (all 54 agents)
  - Startup (6 core agents)
  - Developer (6 tech-focused agents)
  - Minimal (Ali only)

#### Session Management
- **Conversation Persistence** - conversations resume exactly where you left off
- **Thread-based Memory** - context maintained across sessions
- **Agent-specific History** - each agent remembers past interactions

#### Accessibility
- **Keyboard Navigation** - full accessibility with arrow keys
- **Vim Bindings** - j/k navigation for vim users
- **Focus Management** - proper focus handling throughout the panel

#### Git Graph Visualization
- **Commit Graph** - visual representation of repository history
- **Branch Lanes** - color-coded branch visualization
- **Virtual Scrolling** - performant rendering for large repositories
- **Keyboard Navigation** - navigate commits with arrow keys
- **Double-click Actions** - activate commits for details

### Base Editor
- Based on Zed v0.219.0

---

## Release Alignment

This changelog tracks Convergio-specific features only. For base Zed changes,
see the upstream [Zed Changelog](https://zed.dev/releases).

### Zed Version Mapping

| Convergio Version | Zed Base Version | Notes |
|-------------------|------------------|-------|
| 0.1.0 | 0.219.0 | Initial release |
