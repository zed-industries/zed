# Testing Brother IDE AI

## Overview
Brother IDE AI is a fork of Zed with local AI integration. Testing covers:
1. FastAPI backend (`app.py`) — 6 API endpoints
2. CLI tool (`scripts/ask-brother`)
3. Rust branding changes ("Zed" → "Brother IDE AI")
4. Installer script (`install-brother.sh`)
5. Editor task definitions (`.zed/tasks.json`)

## Prerequisites

### Python Dependencies
```bash
pip install fastapi uvicorn httpx
```

### Devin Secrets Needed
None — everything runs locally with no external services.

## Testing the API

### Start the API
```bash
cd <repo-root>
python3 -m uvicorn app:app --host 0.0.0.0 --port 8001 &
```

### Key Endpoints
| Endpoint | Method | Purpose |
|---|---|---|
| `/health` | GET | Health check, reports Ollama connectivity |
| `/v1/omni/reason?request=...` | GET | General AI reasoning |
| `/v1/translate/nl2cmd` | POST | NL → shell command (JSON body: `{"input": "..."}`) |
| `/v1/package/install` | POST | Package install (JSON body: `{"package_name": "..."}`) |
| `/v1/security/scan` | POST | Security scan (JSON body: `{"input": "..."}`) |
| `/v1/models` | GET | List Ollama models |

### Without Ollama
When Ollama is not running (common in CI/testing environments):
- `/health` returns `{"status": "ok", "ollama": "disconnected"}` — this is expected
- AI endpoints return HTTP 503 with `"Ollama is not running. Please start it with: ollama serve"` — this is the correct error-handling behavior
- Validation endpoints (422 errors) work regardless of Ollama status

### With Ollama
If Ollama is available:
```bash
ollama serve &
ollama pull deepseek-r1:7b
```
Then AI endpoints will return actual model responses.

## Testing the CLI

The `scripts/ask-brother` script must be executable (`chmod +x`).

```bash
# Test help output
./scripts/ask-brother --help

# Test API-down detection (stop API first)
./scripts/ask-brother --auto-confirm "test"  # Should exit 1 with error

# Test with running API (and Ollama)
./scripts/ask-brother --auto-confirm "What is Python?"
```

Key flags: `--auto-confirm`, `--translate`, `--security`, `--reason`, `--endpoint`, `--timeout`

## Verifying Rust Branding

Since building Zed takes 20-30 minutes, verify branding via source grep:

```bash
# Should return 0 matches (old strings removed)
grep '"Zed"' crates/release_channel/src/lib.rs | grep -v "//"
grep '"About Zed"' crates/zed/src/zed/app_menus.rs
grep '"Welcome to Zed"' crates/workspace/src/welcome.rs
grep '"Welcome to Zed"' crates/onboarding/src/onboarding.rs

# Should return matches (new strings present)
grep '"Brother IDE AI' crates/release_channel/src/lib.rs
grep '"Brother IDE AI' crates/zed/src/zed/app_menus.rs
```

For a full build check: `cargo check -p release_channel` is fast and verifies the branding crate compiles.

## Verifying Editor Tasks

```bash
# All 5 tasks should be present
grep '"Brother:' .zed/tasks.json
```

Expected: Ask AI, Explain This Code, Generate Unit Tests, Find Security Issues, Translate to Shell Command

## Verifying Installer

```bash
bash -n install-brother.sh  # Syntax check only, no execution
```

## Tips
- The `/v1/package/install` endpoint runs `sudo apt-get install` — skip in automated testing to avoid system modifications
- Port 8001 might already be in use from a previous session; check with `lsof -i :8001`
- The API auto-detects the system package manager (apt-get/dnf/pacman/pip3)
- `cargo check -p release_channel` is much faster than a full workspace build for verifying Rust changes compile
