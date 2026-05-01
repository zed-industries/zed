# Changelog

All notable changes to Brother IDE AI are documented in this file.

## [1.0.0] - 2026-05-01

### Branding

- Renamed all user-facing strings from "Zed" to "Brother IDE AI" across:
  - Release channel display names (`ReleaseChannel::display_name()`)
  - Application menu name and "About" dialog
  - Welcome screens (workspace, onboarding, AI onboarding)
  - Windows product metadata (`FileDescription`, `ProductName`)
  - Context server / MCP client identification
  - Debug adapter protocol client name
  - ETW tracing instance name
  - Desktop entry and Flatpak metainfo descriptions
  - Package description in `Cargo.toml`

### AI Integration

- Added FastAPI backend (`app.py`) with local AI endpoints:
  - `GET /v1/omni/reason` - General AI reasoning via Ollama/DeepSeek
  - `POST /v1/translate/nl2cmd` - Natural language to shell command translation
  - `POST /v1/package/install` - Intelligent package installation
  - `POST /v1/security/scan` - AI-powered security vulnerability scanning
  - `GET /v1/models` - List available Ollama models
  - `GET /health` - Health check with Ollama connectivity status

### Editor Commands

- Added built-in Zed task definitions for AI commands:
  - `Brother: Ask AI` - General AI assistant
  - `Brother: Explain This Code` - Code explanation for selected text
  - `Brother: Generate Unit Tests` - Test generation for selected functions
  - `Brother: Find Security Issues` - Security scanning of selected code
  - `Brother: Translate to Shell Command` - NLP command translation

### CLI

- Added `ask-brother` CLI script (`scripts/ask-brother`) with:
  - `--auto-confirm` flag to skip confirmation prompts
  - `--translate` endpoint for NL-to-command conversion
  - `--security` endpoint for code vulnerability scanning
  - `--reason` endpoint for general AI queries (default)
  - Stdin support for piping code
  - API health check with helpful error messages

### Installer

- Added `install-brother.sh` one-command installer that:
  - Installs system dependencies (apt/dnf/pacman)
  - Installs Rust toolchain
  - Installs and configures Ollama
  - Pulls DeepSeek model
  - Clones and builds the editor
  - Sets up Python virtual environment and API dependencies
  - Installs the `ask-brother` CLI
  - Creates default editor configuration
  - Starts all services

### Documentation

- Rewrote `README.md` with:
  - Quick start guide
  - Detailed manual setup instructions
  - API endpoint documentation
  - Editor command reference
  - CLI usage guide
  - Configuration reference
  - Architecture overview

### Configuration

- Telemetry disabled by default
- Auto-updates blocked by default
- Default settings template includes AI task definitions

### Security

- No external API calls (all AI runs locally via Ollama)
- API error handling with informative messages when Ollama is unavailable
