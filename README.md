# AIDO - AI-powered Command Interface Tool

A Rust-based CLI tool that brings AI-powered command generation to your terminal using Claude Code. Press a hotkey, describe what you want to do in natural language, and let AI generate and execute commands for you.

## Features

- **DO Mode**: Generate and execute shell commands from natural language
- **ASK Mode**: Ask questions and get answers without execution
- **Smart Confirmation**: Preview commands before execution with safety checks
- **Multi-Command Support**: Handle complex multi-step operations
- **Keybinding Support**: Trigger from your terminal with configurable shortcuts (default Ctrl+O / Ctrl+K)
- **Uses Claude CLI**: Leverages `claude -p` non-interactive mode (no API key management needed!)

## Installation

### Homebrew (Recommended)

```bash
brew tap arvin-shafiei/cli-question https://github.com/arvin-shafiei/cli-question
brew install arvin-shafiei/cli-question/aido
```

This installs the `aido` binary into Homebrew's prefix and keeps it up to date with `brew upgrade`.

### Quick Install Script

```bash
cd /Users/arvin/Documents/cli-question
./install.sh
```

The installation script will:
1. Build the project in release mode
2. Install the binary to `~/.local/bin/aido`
3. Add `~/.local/bin` to your PATH in your shell config
4. Initialize the configuration

### Manual Install

```bash
# Build the project
cd aido
cargo build --release

# Copy to a location in your PATH
cp target/release/aido ~/.local/bin/

# Initialize configuration
aido init
```

## Setup

### 1. After Installation

After running the install script, you need to reload your shell configuration:

```bash
# For Zsh
source ~/.zshrc

# For Bash
source ~/.bashrc

# For Fish
source ~/.config/fish/config.fish

# Or just restart your terminal
```

### 2. Set Up Keybindings (Optional but Recommended)

Add keybindings to your shell so you can trigger AIDO with the defaults (Ctrl+O for ASK, Ctrl+K for DO) or your preferred shortcuts:

**Option 1: One-time in current session**
```bash
eval "$(aido setup-shell)"
```

**Option 2: Permanent setup**

Add this line to your shell config file:

For Zsh (`~/.zshrc`) or Bash (`~/.bashrc`):
```bash
# AIDO keybindings
eval "$(aido setup-shell)"
```

For Fish (`~/.config/fish/config.fish`):
```fish
# AIDO keybindings
aido setup-shell | source
```

Then reload your shell config.

## Usage

### Command Line Usage

**DO Mode - Generate and execute commands:**
```bash
aido do "find all PDF files larger than 10MB"
aido do "create a new React component named Button"
aido do "show git commits from last week"
```

**ASK Mode - Ask questions:**
```bash
aido ask "what does rsync do?"
aido ask "how to use grep with regex?"
aido ask "explain the difference between TCP and UDP"
```

**Options:**
```bash
aido do "command" -y          # Skip confirmation, auto-execute
aido do "command" -n          # Dry run, don't execute
aido do "command" -v          # Verbose logging
```

### Keybinding Usage

Once you've set up the shell integration (either via the install script or `aido setup-shell`):

- **Ctrl+O**: Trigger ASK mode (ask questions)
- **Ctrl+K**: Trigger DO mode (generate and execute commands)
- Run `aido` with no arguments to open the settings menu and customize the bindings or unsafe mode.

Just press the key combination in your terminal, type your request, and press Enter!

> **Note:** Defaults are Ctrl+O (ASK) and Ctrl+K (DO), which avoid conflicts with common terminal bindings. You can change them from the interactive settings menu (`aido`).

## Configuration

Configuration file is located at:
- macOS/Linux: `~/.config/aido/config.toml`

### View current config:
```bash
aido config show
```

### Edit config:
```bash
aido config edit
```

### Default Configuration:
```toml
[ai]
claude_command = "claude"
model = "claude-sonnet-4-20250514"

[execution]
always_confirm = true
auto_explain = false
dangerous_commands = ["rm", "mv", "dd", "mkfs"]
dry_run = false
unsafe_mode = false

[ui]
style = "terminal"
color_scheme = "auto"
show_context = true

[keybindings]
ask_binding = "ctrl-o"
do_binding = "ctrl-k"
```

## Commands

| Command | Description |
|---------|-------------|
| `aido do "<task>"` | Generate and execute shell commands |
| `aido ask "<question>"` | Ask questions and get answers |
| `aido init` | Initialize or reset configuration |
| `aido doctor` | Check configuration and dependencies |
| `aido config show` | Display current configuration |
| `aido config edit` | Open config file in editor |
| `aido setup-shell` | Generate shell integration code |
| `aido --help` | Show help message |
| `aido --version` | Show version |

## How It Works

AIDO uses the Claude Code CLI (`claude -p`) in non-interactive mode to generate responses. This means:

1. **No API Key Management**: Uses your existing Claude Code installation and authentication
2. **Simple & Secure**: No need to store API keys in config files
3. **Same Quality**: Gets the same high-quality responses from Claude

### Architecture

```
User Input → AIDO → claude -p → Claude AI → Command/Answer
                ↓
         Safety Check
                ↓
         User Confirmation
                ↓
         Execute Command
```

## Safety Features

AIDO includes several safety features:

1. **Dangerous Command Detection**: Blocks obviously dangerous commands
2. **Confirmation Prompts**: Always asks before executing (unless `-y` flag)
3. **Dry Run Mode**: Preview what would be executed with `-n` flag
4. **Command Explanation**: Can explain what a command does before running it

Blocked patterns include:
- `rm -rf /`
- `dd` to disk devices
- Filesystem formatting commands
- Fork bombs
- Piping curl/wget to shell

## Troubleshooting

### aido command not found

After installation, make sure to reload your shell:
```bash
source ~/.zshrc  # or ~/.bashrc
```

Or check that `~/.local/bin` is in your PATH:
```bash
echo $PATH | grep ".local/bin"
```

### Claude CLI not found

Make sure Claude Code CLI is installed:
```bash
claude --version
```

If not installed, visit: https://docs.claude.com/claude-code

### Keybindings not working

1. Make sure you've added the shell integration to your config file
2. Reload your shell config: `source ~/.zshrc`
3. Test that aido works from command line first: `aido do "test"`

### Check your setup

Run the diagnostics:
```bash
aido doctor
```

This will check:
- Configuration is valid
- Claude CLI is available
- Shell is detected correctly

## Examples

```bash
# File operations
aido do "find all files modified in the last 24 hours"
aido do "create a backup of my Documents folder"

# Git operations
aido do "show git diff for the last commit"
aido do "create a new branch called feature/new-button"

# System operations
aido do "show disk usage for each directory"
aido do "find processes using port 8080"

# Questions
aido ask "what is the difference between grep and awk?"
aido ask "how to debug network issues on macOS?"
```

## Development

### Build from source
```bash
cd aido
cargo build --release
```

### Run tests
```bash
cargo test
```

### Run with logging
```bash
RUST_LOG=debug cargo run -- do "test command"
```

## Requirements

- Rust 1.70+ (for building)
- Claude Code CLI installed and authenticated
- Unix-like system (macOS, Linux)
- Bash, Zsh, or Fish shell (for keybindings)

## License

MIT License - See LICENSE file for details

## Credits

Built with:
- Rust
- Claude Code CLI
- Various amazing Rust crates (see Cargo.toml)

---

**Happy commanding!** 🚀
