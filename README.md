# CodeOrbit

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?logo=typescript&logoColor=white)](https://www.typescriptlang.org/)
[![Node.js](https://img.shields.io/badge/Node.js-339933?logo=node.js&logoColor=white)](https://nodejs.org/)
[![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)](https://www.rust-lang.org/)

Welcome to **CodeOrbit**, a high-performance, AI-powered code editor with multi-agent collaboration capabilities. Built with Rust for maximum performance and designed for developer productivity.

<div align="center">
  <img src="assets/logo.codeorbit.svg" alt="CodeOrbit Logo" width="300"/>
</div>

## ✨ Features

- **Blazing Fast** - Built with Rust for maximum performance
- **AI-Powered** - Smart code completion and assistance
- **Multi-Agent System** - Collaborative AI agents for different development tasks
- **Real-time Collaboration** - Work together with your team in real-time
- **Extensible** - Built with plugins and extensions in mind
- **Cross-Platform** - Available on Windows, macOS, and Linux

## 🚀 Getting Started

### Prerequisites

- **For Development:**
  - Node.js 16.0.0 or higher
  - npm or yarn
  - Git
  - Rust (latest stable)
  - Platform-specific build tools:
    - **Windows:** Visual Studio 2019/2022 with C++ build tools
    - **macOS:** Xcode command line tools
    - **Linux:** Build essentials, GTK+3, and other system dependencies

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/codeorbit.git
   cd codeorbit
   ```

2. Install dependencies:
   ```bash
   npm install
   # or
   yarn
   ```

3. Start the development server:
   ```bash
   npm start
   # or
   yarn start
   ```

## 🛠 Building from Source

### Development Build

```bash
# Clone the repository
https://github.com/yourusername/codeorbit.git
cd codeorbit

# Install dependencies
cargo build

# Run in development mode
cargo run
```

### Release Builds

#### Windows
```bash
# Build for Windows
cargo build --release --target x86_64-pc-windows-msvc

# The binary will be at:
# target/x86_64-pc-windows-msvc/release/zed.exe
```

#### macOS (Intel)
```bash
# Build for Intel Mac
cargo build --release --target x86_64-apple-darwin

# The binary will be at:
# target/x86_64-apple-darwin/release/zed
```

#### macOS (Apple Silicon)
```bash
# Build for Apple Silicon
cargo build --release --target aarch64-apple-darwin

# The binary will be at:
# target/aarch64-apple-darwin/release/zed
```

#### Linux
```bash
# Install system dependencies (Ubuntu/Debian)
sudo apt-get update
sudo apt-get install -y \
    libgtk-3-dev \
    libxcb-render0-dev \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev \
    libspeechd-dev \
    libxkbcommon-dev \
    libssl-dev \
    libgtk-3-0 \
    libwebkit2gtk-4.0-dev \
    libappindicator3-dev \
    librsvg2-dev

# Build for Linux
cargo build --release --target x86_64-unknown-linux-gnu

# The binary will be at:
# target/x86_64-unknown-linux-gnu/release/zed
```

## 📦 Release Process

### Creating a New Release

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` with the new version
3. Commit changes with message `chore: prepare vX.Y.Z release`
4. Create a new Git tag: `git tag vX.Y.Z`
5. Push the tag: `git push origin vX.Y.Z`

### CI/CD Pipeline

The GitHub Actions workflow will automatically:
- Run tests on all platforms
- Build release artifacts for all supported platforms
- Create a GitHub release with all artifacts when a new tag is pushed

### Manual Build Scripts

For local builds, use the provided scripts:

#### Windows (PowerShell)
```powershell
.\scripts\build-release.ps1 -Target windows
```

#### All Platforms
```powershell
.\scripts\build-release.ps1 -Target all
```

#### Specific Target
```powershell
.\scripts\build-release.ps1 -Target x86_64-unknown-linux-gnu
```

## 📦 Release Artifacts

Each release includes:
- `zed-windows.zip` - Windows x86_64 installer
- `zed-macos-x64.tar.gz` - macOS x86_64 binary
- `zed-macos-arm64.tar.gz` - macOS ARM64 binary
- `zed-linux-x64.tar.gz` - Linux x86_64 binary
- `SHA256SUMS` - Checksums for all release artifacts

## 🛠 Development

### Available Scripts

- `npm start` - Start the development server
- `npm run build` - Build for production
- `npm test` - Run tests
- `npm run lint` - Lint the codebase
- `npm run format` - Format the code

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guidelines](CONTRIBUTING.md) to get started.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🌐 Links

- [Documentation](https://docs.codeorbit.dev)
- [GitHub Repository](https://github.com/yourusername/codeorbit)
- [Issue Tracker](https://github.com/yourusername/codeorbit/issues)
- [Changelog](CHANGELOG.md)

## 🙏 Acknowledgments

- Built with ❤️ by the CodeOrbit Team
- Inspired by modern code editors and IDEs
- Thanks to all contributors who help make this project better!
