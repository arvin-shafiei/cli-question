#!/bin/bash

set -e

echo "=== AIDO Installation Script ==="
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get the directory where the script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
AIDO_DIR="$SCRIPT_DIR/aido"

# Check if we're in the right directory
if [ ! -d "$AIDO_DIR" ]; then
    echo -e "${RED}✗ Error: aido directory not found!${NC}"
    echo "Make sure you're running this script from the cli-question directory."
    exit 1
fi

# Build the project
echo "Building aido..."
cd "$AIDO_DIR"
cargo build --release

if [ $? -ne 0 ]; then
    echo -e "${RED}✗ Build failed!${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Build successful${NC}"

# Create install directory
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

# Copy binary
echo "Installing aido to $INSTALL_DIR..."
cp target/release/aido "$INSTALL_DIR/aido"
chmod +x "$INSTALL_DIR/aido"

echo -e "${GREEN}✓ Binary installed to $INSTALL_DIR/aido${NC}"

# Detect shell
SHELL_NAME=$(basename "$SHELL")
SHELL_RC=""

case "$SHELL_NAME" in
    bash)
        SHELL_RC="$HOME/.bashrc"
        ;;
    zsh)
        SHELL_RC="$HOME/.zshrc"
        ;;
    fish)
        SHELL_RC="$HOME/.config/fish/config.fish"
        ;;
    *)
        echo -e "${YELLOW}⚠ Unknown shell: $SHELL_NAME${NC}"
        echo "Please manually add $INSTALL_DIR to your PATH"
        ;;
esac

# Add to PATH if not already there
if [ -n "$SHELL_RC" ]; then
    if ! grep -q "$INSTALL_DIR" "$SHELL_RC" 2>/dev/null; then
        echo ""
        echo "Adding $INSTALL_DIR to PATH in $SHELL_RC..."

        if [ "$SHELL_NAME" = "fish" ]; then
            echo "" >> "$SHELL_RC"
            echo "# Added by aido installer" >> "$SHELL_RC"
            echo "set -gx PATH $INSTALL_DIR \$PATH" >> "$SHELL_RC"
        else
            echo "" >> "$SHELL_RC"
            echo "# Added by aido installer" >> "$SHELL_RC"
            echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$SHELL_RC"
        fi

        echo -e "${GREEN}✓ PATH updated in $SHELL_RC${NC}"
    else
        echo -e "${GREEN}✓ $INSTALL_DIR already in PATH${NC}"
    fi
fi

# Initialize aido config
echo ""
echo "Initializing aido configuration..."
"$INSTALL_DIR/aido" init

# Add shell keybindings
if [ -n "$SHELL_RC" ]; then
    if ! grep -q "aido setup-shell" "$SHELL_RC" 2>/dev/null; then
        echo ""
        echo "Adding AIDO keybindings to $SHELL_RC..."
        echo "" >> "$SHELL_RC"
        echo "# AIDO keybindings - added automatically" >> "$SHELL_RC"
        echo 'eval "$(aido setup-shell)"' >> "$SHELL_RC"
        echo -e "${GREEN}✓ Keybindings added to $SHELL_RC${NC}"
    else
        echo -e "${GREEN}✓ Keybindings already configured${NC}"
    fi
fi

echo ""
echo -e "${GREEN}=== Installation Complete! ===${NC}"
echo ""

# Reload shell configuration
if [ -n "$SHELL_RC" ] && [ -f "$SHELL_RC" ]; then
    echo "Reloading shell configuration..."
    if [ "$SHELL_NAME" = "zsh" ]; then
        # For zsh, we can source it in the current shell
        echo -e "${YELLOW}Please run: source $SHELL_RC${NC}"
    elif [ "$SHELL_NAME" = "bash" ]; then
        echo -e "${YELLOW}Please run: source $SHELL_RC${NC}"
    fi
    echo ""
fi

echo "To start using aido:"
echo "  1. Run: source $SHELL_RC  (to activate keybindings)"
echo "  2. Try: aido do \"list all files\""
echo "  3. Try: aido ask \"what does rsync do?\""
echo ""
echo "Keybindings:"
echo "  - Ctrl+O: DO mode (generate and execute commands)"
echo "  - Ctrl+Alt+O: ASK mode (ask questions)"
