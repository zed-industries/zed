# CodeOrbit - AI-Powered Development Assistant for Zed

![CodeOrbit Logo](./assets/logo.codeorbit.svg)

CodeOrbit is an intelligent code assistant extension for Zed that provides AI-powered development tools, multi-agent collaboration, and context-aware coding assistance.

## Features

- 🚀 AI-Powered Code Completion & Generation
- 🤖 Multi-Agent System for different development tasks
- 🧠 Context-Aware Development Environment
- 🛠️ Built-in Development Tools & Utilities
- 🔌 Extensible Architecture for Custom Agents

## Installation

### Prerequisites

- [Zed Editor](https://zed.dev/)
- Rust toolchain (latest stable)
- Cargo

### Building from Source

1. Clone the repository:
   ```bash
   git clone https://github.com/your-org/codeorbit-zed.git
   cd codeorbit-zed/codeorbit
   ```

2. Build the extension:
   ```bash
   cargo build --release
   ```

3. Install the extension in Zed:
   - Open Zed
   - Open the command palette (Cmd/Ctrl+Shift+P)
   - Run "Install Extension"
   - Select the `target/release/libcodeorbit.so` (Linux), `target/release/libcodeorbit.dylib` (macOS), or `target/release/codeorbit.dll` (Windows) file

## Usage

Once installed, you can access CodeOrbit features through:

- **Command Palette**: Press `Cmd/Ctrl+Shift+P` and search for "CodeOrbit"
- **Keyboard Shortcut**: `Ctrl+Shift+O` (configurable in Zed settings)
- **Context Menu**: Right-click in the editor for context-aware actions

## Configuration

CodeOrbit can be configured via the `config.toml` file. The following options are available:

```toml
[core]
enabled = true
log_level = "info"

[ai]
model = "gpt-4"
max_tokens = 2048
temperature = 0.7

[ui]
show_welcome = true
panel_position = "right"
panel_width = 400
panel_height = 300
```

## Development

### Project Structure

```
codeorbit/
├── src/                 # Rust source code
├── agents/              # Agent implementations
│   ├── frontend/       # Frontend-related agents
│   ├── backend/        # Backend-related agents
│   ├── database/       # Database-related agents
│   ├── devops/         # DevOps-related agents
│   └── docs/           # Documentation-related agents
├── core/               # Core functionality
├── ui/                 # UI components
├── assets/             # Static assets
├── Cargo.toml          # Rust project configuration
├── zed.toml            # Zed extension manifest
└── config.toml         # Extension configuration
```

### Building and Testing

```bash
# Build in debug mode
cargo build

# Build in release mode
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt

# Check for clippy warnings
cargo clippy
```

## Prompt Handling Loop

The extension provides a simple round-trip for user prompts. A prompt entered in
the UI is sent to the orchestrator, which forwards it to the `UiPlannerAgent`.
The agent returns a UI component plan and the orchestrator delivers this back to
the prompt panel for display.

Submit a prompt by pressing **Enter** inside the panel's input area or by
clicking the *Send* button. Any agent errors are shown inline.

Main files involved:

- `ui/prompt_panel.rs` – gathers user input and renders responses.
- `core/orchestrator.rs` – routes prompts and manages agents.
- `agents/frontend/ui_planner_agent.rs` – interprets prompts and creates a UI plan.


## Contributing

Contributions are welcome! Please read our [Contributing Guidelines](CONTRIBUTING.md) for details.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

For support, please open an issue in our issue tracker or join our community chat.
