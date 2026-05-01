#!/usr/bin/env bash
set -euo pipefail

# Brother IDE AI - Installation Script
# This script installs and configures Brother IDE AI with local AI integration.
# It sets up Ollama, pulls the DeepSeek model, compiles the editor, and starts the API.

REPO_URL="https://github.com/abdoulayecoumbassa74-design/zed.git"
INSTALL_DIR="${BROTHER_INSTALL_DIR:-$HOME/brother-ide-ai}"
API_PORT="${BROTHER_API_PORT:-8001}"
MODEL="${BROTHER_MODEL:-deepseek-r1:7b}"
VENV_DIR="$INSTALL_DIR/.venv"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info()  { echo -e "${BLUE}[INFO]${NC} $*"; }
log_ok()    { echo -e "${GREEN}[OK]${NC} $*"; }
log_warn()  { echo -e "${YELLOW}[WARN]${NC} $*"; }
log_error() { echo -e "${RED}[ERROR]${NC} $*"; }

check_command() {
    if ! command -v "$1" &>/dev/null; then
        return 1
    fi
    return 0
}

install_system_deps() {
    log_info "Installing system dependencies..."
    if check_command apt-get; then
        sudo apt-get update -qq
        sudo apt-get install -y -qq \
            build-essential \
            curl \
            git \
            pkg-config \
            libssl-dev \
            libfontconfig-dev \
            libwayland-dev \
            libxkbcommon-dev \
            libvulkan-dev \
            python3 \
            python3-pip \
            python3-venv \
            cmake
    elif check_command dnf; then
        sudo dnf install -y \
            gcc gcc-c++ make \
            curl git \
            pkg-config openssl-devel \
            fontconfig-devel \
            wayland-devel libxkbcommon-devel \
            vulkan-loader-devel \
            python3 python3-pip \
            cmake
    elif check_command pacman; then
        sudo pacman -Syu --noconfirm \
            base-devel curl git \
            pkg-config openssl \
            fontconfig wayland \
            libxkbcommon vulkan-icd-loader \
            python python-pip cmake
    else
        log_error "Unsupported package manager. Please install dependencies manually."
        exit 1
    fi
    log_ok "System dependencies installed."
}

install_rust() {
    if check_command rustup; then
        log_info "Rust already installed, updating..."
        rustup update stable
    else
        log_info "Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        # shellcheck source=/dev/null
        source "$HOME/.cargo/env"
    fi
    log_ok "Rust is ready: $(rustc --version)"
}

install_ollama() {
    if check_command ollama; then
        log_info "Ollama already installed."
    else
        log_info "Installing Ollama..."
        curl -fsSL https://ollama.com/install.sh | sh
    fi
    log_ok "Ollama is ready."
}

pull_model() {
    log_info "Pulling model $MODEL (this may take a while)..."
    ollama pull "$MODEL"
    log_ok "Model $MODEL is ready."
}

clone_repo() {
    if [ -d "$INSTALL_DIR/.git" ]; then
        log_info "Repository already exists at $INSTALL_DIR, pulling latest..."
        git -C "$INSTALL_DIR" pull --ff-only || true
    else
        log_info "Cloning Brother IDE AI repository..."
        git clone "$REPO_URL" "$INSTALL_DIR"
    fi
    log_ok "Repository ready at $INSTALL_DIR."
}

build_editor() {
    log_info "Building Brother IDE AI editor (this may take 10-30 minutes)..."
    cd "$INSTALL_DIR"
    cargo build --release -p zed 2>&1 | tail -5
    log_ok "Brother IDE AI built successfully."
}

setup_api() {
    log_info "Setting up Brother IDE AI API..."

    python3 -m venv "$VENV_DIR"
    # shellcheck source=/dev/null
    source "$VENV_DIR/bin/activate"
    pip install --quiet fastapi uvicorn httpx

    log_ok "API dependencies installed in virtual environment."
}

install_ask_brother_script() {
    log_info "Installing ask-brother CLI script..."
    mkdir -p "$HOME/.local/bin"

    cp "$INSTALL_DIR/scripts/ask-brother" "$HOME/.local/bin/ask-brother"
    chmod +x "$HOME/.local/bin/ask-brother"

    if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
        log_warn "Add $HOME/.local/bin to your PATH:"
        echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
    fi

    log_ok "ask-brother installed to $HOME/.local/bin/ask-brother"
}

copy_config() {
    log_info "Setting up default configuration..."
    local config_dir="$HOME/.config/zed"
    mkdir -p "$config_dir"

    if [ ! -f "$config_dir/settings.json" ]; then
        cat > "$config_dir/settings.json" << 'SETTINGS'
{
  "telemetry": {
    "diagnostics": false,
    "metrics": false
  },
  "auto_update": false,
  "features": {
    "copilot": false
  },
  "tasks": [
    {
      "label": "Ask Brother AI",
      "command": "ask-brother",
      "args": ["$ZED_SELECTED_TEXT"],
      "use_new_terminal": false,
      "reveal": "always"
    },
    {
      "label": "Explain This Code",
      "command": "curl",
      "args": ["-s", "-G", "--data-urlencode", "request=Explain this code: $ZED_SELECTED_TEXT", "http://localhost:8001/v1/omni/reason"],
      "use_new_terminal": false,
      "reveal": "always"
    },
    {
      "label": "Generate Unit Tests",
      "command": "curl",
      "args": ["-s", "-G", "--data-urlencode", "request=Generate unit tests for: $ZED_SELECTED_TEXT", "http://localhost:8001/v1/omni/reason"],
      "use_new_terminal": false,
      "reveal": "always"
    },
    {
      "label": "Find Security Issues",
      "command": "curl",
      "args": ["-s", "-G", "--data-urlencode", "request=Analyze this code for security vulnerabilities: $ZED_SELECTED_TEXT", "http://localhost:8001/v1/omni/reason"],
      "use_new_terminal": false,
      "reveal": "always"
    }
  ]
}
SETTINGS
        log_ok "Default settings.json created."
    else
        log_warn "settings.json already exists, skipping. Review $INSTALL_DIR/docs/settings-example.json for recommended settings."
    fi
}

start_services() {
    log_info "Starting services..."

    if ! pgrep -f "ollama serve" > /dev/null 2>&1; then
        log_info "Starting Ollama server..."
        nohup ollama serve > /tmp/ollama.log 2>&1 &
        sleep 2
    fi

    if ! curl -s "http://localhost:$API_PORT/health" > /dev/null 2>&1; then
        log_info "Starting Brother IDE AI API on port $API_PORT..."
        cd "$INSTALL_DIR"
        # shellcheck source=/dev/null
        source "$VENV_DIR/bin/activate"
        nohup uvicorn app:app --host 0.0.0.0 --port "$API_PORT" > /tmp/brother-api.log 2>&1 &
        sleep 2
    fi

    if curl -s "http://localhost:$API_PORT/health" > /dev/null 2>&1; then
        log_ok "API is running on http://localhost:$API_PORT"
    else
        log_warn "API may not have started correctly. Check /tmp/brother-api.log"
    fi
}

print_summary() {
    echo ""
    echo -e "${GREEN}============================================${NC}"
    echo -e "${GREEN}  Brother IDE AI - Installation Complete!${NC}"
    echo -e "${GREEN}============================================${NC}"
    echo ""
    echo "  Editor binary:  $INSTALL_DIR/target/release/zed"
    echo "  API endpoint:   http://localhost:$API_PORT"
    echo "  AI Model:       $MODEL"
    echo "  Config:         $HOME/.config/zed/settings.json"
    echo ""
    echo "  Quick start:"
    echo "    $INSTALL_DIR/target/release/zed"
    echo ""
    echo "  Test AI:"
    echo "    curl 'http://localhost:$API_PORT/v1/omni/reason?request=Hello'"
    echo "    ask-brother 'What is Python?'"
    echo ""
    echo "  Logs:"
    echo "    Ollama:  tail -f /tmp/ollama.log"
    echo "    API:     tail -f /tmp/brother-api.log"
    echo ""
}

main() {
    echo -e "${BLUE}"
    echo "  ____            _   _               ___ ____  _____      _    ___"
    echo " | __ ) _ __ ___ | |_| |__   ___ _ __|_ _|  _ \\| ____|   / \\  |_ _|"
    echo " |  _ \\| '__/ _ \\| __| '_ \\ / _ \\ '__|| || | | |  _|    / _ \\  | |"
    echo " | |_) | | | (_) | |_| | | |  __/ |  | || |_| | |___  / ___ \\ | |"
    echo " |____/|_|  \\___/ \\__|_| |_|\\___|_| |___|____/|_____|/_/   \\_\\___|"
    echo -e "${NC}"
    echo ""

    install_system_deps
    install_rust
    install_ollama
    pull_model
    clone_repo
    build_editor
    setup_api
    install_ask_brother_script
    copy_config
    start_services
    print_summary
}

if [[ "${1:-}" == "--help" ]] || [[ "${1:-}" == "-h" ]]; then
    echo "Usage: ./install-brother.sh"
    echo ""
    echo "Environment variables:"
    echo "  BROTHER_INSTALL_DIR  Installation directory (default: ~/brother-ide-ai)"
    echo "  BROTHER_API_PORT     API port (default: 8001)"
    echo "  BROTHER_MODEL        AI model to use (default: deepseek-r1:7b)"
    exit 0
fi

main "$@"
