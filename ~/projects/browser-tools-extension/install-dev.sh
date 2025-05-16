#!/bin/bash
set -e

# Color codes for better output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Browser Tools Dev Extension Installer${NC}"
echo "This script will help install the Browser Tools extension for development in Zed"

# Check if Zed is installed
if ! command -v zed &> /dev/null; then
    echo -e "${RED}Error: Zed editor not found in your PATH${NC}"
    echo "Please make sure Zed is installed and accessible from your terminal"
    exit 1
fi

# Get extension directory (absolute path)
EXTENSION_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
SERVER_DIR=$(cd "$EXTENSION_DIR/../browser_tools_server" && pwd)

echo -e "${GREEN}Extension directory:${NC} $EXTENSION_DIR"
echo -e "${GREEN}Server directory:${NC} $SERVER_DIR"

# Clean previous builds
echo -e "\n${YELLOW}Cleaning previous builds...${NC}"
cd "$EXTENSION_DIR"
cargo clean
mkdir -p "$EXTENSION_DIR/target/wasm32-wasip1/debug"

# Build the server package first
echo -e "\n${YELLOW}Building browser_tools_server package...${NC}"
cd "$SERVER_DIR"
cargo build

if [ $? -ne 0 ]; then
    echo -e "${RED}Server build failed. Please fix the errors and try again.${NC}"
    exit 1
fi
echo -e "${GREEN}Server build successful!${NC}"

# Check for wasm32-wasip1 target
if ! rustup target list --installed | grep -q "wasm32-wasip1"; then
    echo -e "${YELLOW}Adding wasm32-wasip1 target...${NC}"
    rustup target add wasm32-wasip1
fi

# Build the extension
echo -e "\n${YELLOW}Building extension...${NC}"
cd "$EXTENSION_DIR"
cargo build --target wasm32-wasip1

if [ $? -ne 0 ]; then
    echo -e "${RED}Extension build failed. Please fix the errors and try again.${NC}"
    exit 1
fi
echo -e "${GREEN}Extension build successful!${NC}"

# Verify WebAssembly module
EXTENSION_WASM_FILE="$EXTENSION_DIR/target/wasm32-wasip1/debug/browser_tools.wasm"

if [ -f "$EXTENSION_WASM_FILE" ]; then
    echo -e "${GREEN}WebAssembly module found:${NC} $EXTENSION_WASM_FILE"
else
    echo -e "${RED}WebAssembly module not found. Searching for any WASM files...${NC}"
    # Try to find any WASM file
    FOUND_WASM=$(find "$HOME/.cargo" -name "browser_tools.wasm" | head -1)
    
    if [ -n "$FOUND_WASM" ]; then
        echo -e "${GREEN}Found WASM file:${NC} $FOUND_WASM"
        echo -e "${YELLOW}Copying to expected location...${NC}"
        mkdir -p "$(dirname "$EXTENSION_WASM_FILE")"
        cp "$FOUND_WASM" "$EXTENSION_WASM_FILE"
        echo -e "${GREEN}WebAssembly module copied to:${NC} $EXTENSION_WASM_FILE"
    else
        echo -e "${RED}No WASM files found. Build likely failed completely.${NC}"
        exit 1
    fi
fi

# Get Zed extensions directory
ZED_EXTENSIONS_DIR="$HOME/.config/zed/extensions"
mkdir -p "$ZED_EXTENSIONS_DIR"
echo -e "\n${GREEN}Zed extensions directory:${NC} $ZED_EXTENSIONS_DIR"

# Create appropriate extension directory structure
DEV_EXTENSION_DIR="$ZED_EXTENSIONS_DIR/dev.browser-tools"
echo -e "\n${YELLOW}Creating dev extension directory...${NC}"

# Remove existing extension if it exists
if [ -d "$DEV_EXTENSION_DIR" ] || [ -L "$DEV_EXTENSION_DIR" ]; then
    echo "Removing existing dev extension..."
    rm -rf "$DEV_EXTENSION_DIR"
fi

# Create extension directory with expected structure
mkdir -p "$DEV_EXTENSION_DIR"
cp "$EXTENSION_DIR/extension.toml" "$DEV_EXTENSION_DIR/"
cp "$EXTENSION_WASM_FILE" "$DEV_EXTENSION_DIR/browser_tools.wasm"

echo -e "${GREEN}Dev extension created at:${NC} $DEV_EXTENSION_DIR"

# If regular extension already installed, try to uninstall it first
echo -e "\n${YELLOW}Checking for existing installation...${NC}"
if [ -d "$ZED_EXTENSIONS_DIR/installed/browser-tools" ]; then
    echo "Found existing installation, will try to clean up first"
    rm -rf "$ZED_EXTENSIONS_DIR/installed/browser-tools"
    echo "Removed existing installation directory"
fi

echo -e "\n${GREEN}Installation complete!${NC}"
echo -e "To use the extension in Zed:"
echo -e "1. Open Zed"
echo -e "2. If Zed is already running, restart it to load the new extension"
echo -e "3. The extension should appear in Extensions panel as 'Browser Tools (dev)'"
echo ""
echo -e "${YELLOW}Note:${NC} The extension name displayed in Zed should be: '${GREEN}Browser Tools (dev)${NC}'" 