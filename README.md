# Brother IDE AI

The most powerful, sovereign, and secure code editor with integrated local AI.

Built on [Zed](https://github.com/zed-industries/zed)'s high-performance foundation, Brother IDE AI adds a fully local AI assistant powered by [Ollama](https://ollama.com) and [DeepSeek](https://github.com/deepseek-ai/DeepSeek-R1) models. No cloud dependency, no telemetry, complete data sovereignty.

---

## Features

- **Local AI Assistant** - Ask Brother AI questions, explain code, generate tests, and scan for security issues, all running locally on your machine
- **High Performance** - Built on Zed's GPU-accelerated rendering engine
- **No Cloud Dependencies** - All AI processing runs locally via Ollama
- **Telemetry Disabled** - No data leaves your machine
- **Security First** - Code execution through SecureExecutor sandbox
- **NLP Command Translation** - Convert natural language to shell commands

---

## Quick Start

### One-Command Installation

```bash
curl -fsSL https://raw.githubusercontent.com/abdoulayecoumbassa74-design/zed/main/install-brother.sh | bash
```

Or clone and run manually:

```bash
git clone https://github.com/abdoulayecoumbassa74-design/zed.git brother-ide-ai
cd brother-ide-ai
./install-brother.sh
```

### Manual Setup

#### 1. Install Ollama

```bash
curl -fsSL https://ollama.com/install.sh | sh
```

#### 2. Pull the DeepSeek Model

```bash
ollama pull deepseek-r1:7b
```

#### 3. Start Ollama

```bash
ollama serve
```

#### 4. Install Python Dependencies

```bash
python3 -m venv .venv
source .venv/bin/activate
pip install fastapi uvicorn httpx
```

#### 5. Start the API

```bash
uvicorn app:app --host 0.0.0.0 --port 8001
```

#### 6. Build Brother IDE AI

See [Building for Linux](./docs/src/development/linux.md), [macOS](./docs/src/development/macos.md), or [Windows](./docs/src/development/windows.md).

```bash
cargo build --release -p zed
```

#### 7. Launch

```bash
./target/release/zed
```

---

## AI Integration

### API Endpoints

The Brother IDE AI API runs locally on `http://localhost:8001`:

| Endpoint | Method | Description |
|---|---|---|
| `/health` | GET | Health check with Ollama status |
| `/v1/omni/reason` | GET | General AI reasoning (query param: `request`) |
| `/v1/translate/nl2cmd` | POST | Natural language to shell command |
| `/v1/package/install` | POST | Install packages via detected package manager |
| `/v1/security/scan` | POST | Scan code for security vulnerabilities |
| `/v1/models` | GET | List available Ollama models |

### Editor Commands (via Task Palette)

Open the task palette in Brother IDE AI and use these built-in tasks:

| Command | Description |
|---|---|
| `Brother: Ask AI` | Ask the AI assistant any question |
| `Brother: Explain This Code` | Explain the selected code |
| `Brother: Generate Unit Tests` | Generate tests for selected function |
| `Brother: Find Security Issues` | Scan selected code for vulnerabilities |
| `Brother: Translate to Shell Command` | Convert selected text to a shell command |

### CLI Tool: `ask-brother`

```bash
# Simple question
ask-brother "What is a closure in Rust?"

# With auto-confirm (no prompt)
ask-brother --auto-confirm "Explain async/await"

# Translate natural language to command
ask-brother --translate "list all python files recursively"

# Pipe code for security scan
cat myfile.py | ask-brother --security "Check this code"
```

### Testing the AI

```bash
# Test the API
curl "http://localhost:8001/v1/omni/reason?request=Hello"

# Test NL to command translation
curl -X POST http://localhost:8001/v1/translate/nl2cmd \
  -H "Content-Type: application/json" \
  -d '{"input": "list all python files"}'

# Check health
curl http://localhost:8001/health
```

---

## Configuration

### Settings (`~/.config/zed/settings.json`)

```json
{
  "telemetry": {
    "diagnostics": false,
    "metrics": false
  },
  "auto_update": false
}
```

### Environment Variables

| Variable | Default | Description |
|---|---|---|
| `BROTHER_API_URL` | `http://localhost:8001` | API endpoint for ask-brother CLI |
| `BROTHER_API_PORT` | `8001` | Port for the API server |
| `BROTHER_MODEL` | `deepseek-r1:7b` | Ollama model to use |
| `OLLAMA_URL` | `http://localhost:11434` | Ollama server URL |
| `BROTHER_TIMEOUT` | `120` | Request timeout in seconds |
| `BROTHER_INSTALL_DIR` | `~/brother-ide-ai` | Installation directory |

---

## Architecture

```
Brother IDE AI
+-- Editor (Rust/GPUI) .............. Modified Zed editor
+-- API (FastAPI/Python) ............ app.py on port 8001
|   +-- /v1/omni/reason ............ General AI reasoning
|   +-- /v1/translate/nl2cmd ....... NL to shell command
|   +-- /v1/package/install ........ Package installation
|   +-- /v1/security/scan .......... Security analysis
+-- Ollama .......................... Local LLM runtime
|   +-- deepseek-r1:7b ............. Default reasoning model
+-- CLI (ask-brother) ............... Command-line interface
+-- install-brother.sh .............. One-command installer
```

---

## Developing

### Building from Source

- [Building for macOS](./docs/src/development/macos.md)
- [Building for Linux](./docs/src/development/linux.md)
- [Building for Windows](./docs/src/development/windows.md)

### Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for ways to contribute.

### Licensing

License information for third party dependencies must be correctly provided for CI to pass. See the original [Zed licensing documentation](https://github.com/zed-industries/zed#licensing) for details.

---

## Credits

Brother IDE AI is built on top of [Zed](https://zed.dev), a high-performance code editor by Zed Industries.
