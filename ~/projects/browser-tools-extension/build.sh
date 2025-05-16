#!/bin/bash
set -e

# Color codes for better output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Building Browser Tools Extension${NC}"

# Get project directories
EXTENSION_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
SERVER_DIR=$(cd "$EXTENSION_DIR/../browser_tools_server" && pwd)

echo -e "${GREEN}Extension directory:${NC} $EXTENSION_DIR"
echo -e "${GREEN}Server directory:${NC} $SERVER_DIR"

# Build the server first
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
echo -e "\n${YELLOW}Building browser-tools extension...${NC}"
cd "$EXTENSION_DIR"
cargo build --target wasm32-wasip1

if [ $? -ne 0 ]; then
    echo -e "${RED}Extension build failed. Please fix the errors and try again.${NC}"
    exit 1
fi

echo -e "${GREEN}Build complete!${NC}"

# Copy WASM file to the extension directory
WASM_PATH="$EXTENSION_DIR/target/wasm32-wasip1/debug/browser_tools.wasm"
if [ -f "$WASM_PATH" ]; then
    echo -e "${YELLOW}Copying WASM file to extension directory...${NC}"
    cp "$WASM_PATH" "$EXTENSION_DIR/extension.wasm"
    echo -e "${GREEN}WASM file copied successfully!${NC}"
else
    echo -e "${RED}WASM file not found at expected location: $WASM_PATH${NC}"
    
    # Check other common locations
    ALTERNATIVE_PATH="$HOME/.cargo/target/wasm32-wasip1/debug/browser_tools.wasm"
    if [ -f "$ALTERNATIVE_PATH" ]; then
        echo -e "${YELLOW}Found WASM file at alternative location, copying...${NC}"
        cp "$ALTERNATIVE_PATH" "$EXTENSION_DIR/extension.wasm"
        echo -e "${GREEN}WASM file copied successfully!${NC}"
    else
        echo -e "${RED}No WASM file found. Build likely failed.${NC}"
        exit 1
    fi
fi

echo -e "\n${GREEN}Browser Tools build completed successfully.${NC}"
echo -e "The extension is ready for installation in Zed." 