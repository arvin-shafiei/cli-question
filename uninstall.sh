#!/bin/bash

set -e

echo "=== AIDO Uninstallation Script ==="
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

INSTALL_DIR="$HOME/.local/bin"
CONFIG_DIR="$HOME/.config/aido"
ALT_CONFIG_DIR="$HOME/Library/Application Support/aido"  # macOS

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
esac

echo "This will remove:"
echo "  - Binary: $INSTALL_DIR/aido"
echo "  - Configuration: $CONFIG_DIR"
if [ -d "$ALT_CONFIG_DIR" ]; then
    echo "  - Configuration: $ALT_CONFIG_DIR"
fi
echo "  - Shell integration from: $SHELL_RC"
echo "  - PATH entry from: $SHELL_RC"
echo ""
read -p "Continue with uninstall? [y/N]: " -n 1 -r
echo ""

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Uninstall cancelled."
    exit 0
fi

# Remove binary
if [ -f "$INSTALL_DIR/aido" ]; then
    echo "Removing binary..."
    rm "$INSTALL_DIR/aido"
    echo -e "${GREEN}✓ Binary removed${NC}"
else
    echo -e "${YELLOW}⚠ Binary not found${NC}"
fi

# Remove config directories
if [ -d "$CONFIG_DIR" ]; then
    echo "Removing config directory: $CONFIG_DIR"
    rm -rf "$CONFIG_DIR"
    echo -e "${GREEN}✓ Config directory removed${NC}"
fi

if [ -d "$ALT_CONFIG_DIR" ]; then
    echo "Removing config directory: $ALT_CONFIG_DIR"
    rm -rf "$ALT_CONFIG_DIR"
    echo -e "${GREEN}✓ Config directory removed${NC}"
fi

# Remove from shell config
if [ -n "$SHELL_RC" ] && [ -f "$SHELL_RC" ]; then
    echo "Removing AIDO entries from $SHELL_RC..."

    # Create a backup
    cp "$SHELL_RC" "${SHELL_RC}.backup.$(date +%Y%m%d_%H%M%S)"
    echo -e "${GREEN}✓ Backup created: ${SHELL_RC}.backup.$(date +%Y%m%d_%H%M%S)${NC}"

    # Remove AIDO-related lines
    # This removes:
    # - The PATH addition
    # - The keybindings setup
    # - Comment lines added by aido

    # Use sed to remove aido-related blocks
    if [ "$SHELL_NAME" = "bash" ] || [ "$SHELL_NAME" = "zsh" ]; then
        # Remove the PATH line
        sed -i.tmp '/# Added by aido installer/d' "$SHELL_RC"
        sed -i.tmp '\|^export PATH=.*\.local/bin.*PATH|d' "$SHELL_RC"

        # Remove the keybindings line
        sed -i.tmp '/# AIDO keybindings/d' "$SHELL_RC"
        sed -i.tmp '/eval.*aido setup-shell/d' "$SHELL_RC"

        # Clean up empty lines (only consecutive ones)
        sed -i.tmp '/^$/N;/^\n$/D' "$SHELL_RC"

        rm -f "${SHELL_RC}.tmp"
    elif [ "$SHELL_NAME" = "fish" ]; then
        # Similar cleanup for fish
        sed -i.tmp '/# Added by aido installer/d' "$SHELL_RC"
        sed -i.tmp '/set -gx PATH.*\.local\/bin/d' "$SHELL_RC"
        sed -i.tmp '/# AIDO keybindings/d' "$SHELL_RC"
        sed -i.tmp '/aido setup-shell/d' "$SHELL_RC"
        sed -i.tmp '/^$/N;/^\n$/D' "$SHELL_RC"
        rm -f "${SHELL_RC}.tmp"
    fi

    echo -e "${GREEN}✓ Shell configuration cleaned${NC}"
fi

echo ""
echo -e "${GREEN}=== Uninstallation Complete! ===${NC}"
echo ""
echo "AIDO has been removed from your system."
echo ""
echo "Note:"
echo "  - Backup of your shell config was created"
echo "  - You may need to restart your terminal"
echo "  - If you added custom AIDO configurations, check the backup file"
echo ""
echo "To complete the cleanup, restart your terminal or run:"
echo "  source $SHELL_RC"
