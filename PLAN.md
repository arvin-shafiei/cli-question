# AI-DO: AI-Powered Command Interface Tool

> Cursor-style Command+K for the terminal - AI-powered command generation and execution

## Overview

A Rust-based CLI tool that brings AI-powered command generation to your terminal. Press a hotkey, describe what you want to do in natural language, and let AI generate and execute the commands for you.

### Core Features

- **DO Mode**: Generate and execute shell commands from natural language
- **ASK Mode**: Ask questions and get answers without execution
- **Global Hotkey**: Trigger from anywhere with Cmd+J (configurable)
- **Smart Confirmation**: Preview commands before execution with safety checks
- **Multi-Command Support**: Handle complex multi-step operations
- **Command Extraction**: Intelligently parse and order commands from AI responses

---

## Project Structure

```
aido/
├── src/
│   ├── main.rs              # Entry point, CLI arg parsing
│   ├── ui/
│   │   ├── mod.rs           # UI module
│   │   ├── input.rs         # Input capture (popup/terminal)
│   │   ├── display.rs       # Output formatting & display
│   │   └── hotkey.rs        # Global hotkey listener (macOS)
│   ├── ai/
│   │   ├── mod.rs           # AI module
│   │   ├── client.rs        # Claude API client
│   │   ├── prompts.rs       # System prompts for different modes
│   │   └── parser.rs        # Command extraction & parsing
│   ├── executor/
│   │   ├── mod.rs           # Execution module
│   │   ├── shell.rs         # Shell command execution
│   │   ├── confirm.rs       # Confirmation workflow
│   │   └── validator.rs     # Safety checks for commands
│   ├── config/
│   │   ├── mod.rs           # Configuration module
│   │   └── settings.rs      # User settings (API key, model, etc.)
│   └── modes/
│       ├── mod.rs           # Mode dispatcher
│       ├── do_mode.rs       # Command generation & execution
│       └── ask_mode.rs      # Q&A mode
├── Cargo.toml
├── README.md
└── PLAN.md
```

---

## User Interaction Flow

### Mode 1: DO Mode (Command Generation & Execution)

```
User: Cmd+J → "find all files larger than 100MB"
        ↓
AI generates: find . -type f -size +100M -exec ls -lh {} \;
        ↓
Display preview:
┌─────────────────────────────────────────┐
│ AI generated command:                    │
│                                          │
│ ▶ find . -type f -size +100M \          │
│     -exec ls -lh {} \;                   │
│                                          │
│ [Y] Execute  [n] Cancel                  │
│ [e] Edit     [x] Explain                 │
└─────────────────────────────────────────┘
        ↓
Execute → Show results
```

### Mode 2: ASK Mode (Question & Answer)

```
User: Cmd+Shift+J → "what does rsync do?"
        ↓
AI responds with explanation
        ↓
Display answer in terminal
        ↓
[Optional] Follow-up questions
```

### Smart Mode Detection

The tool automatically detects user intent:

- **ASK Mode Triggers**: "how", "what", "explain", "why", "when", "?"
- **DO Mode Triggers**: Action verbs like "find", "create", "delete", "install", "list"
- **Manual Override**: `--do` or `--ask` flags

---

## Technical Implementation

### A. Core Dependencies

```toml
[dependencies]
# CLI & Args
clap = { version = "4", features = ["derive"] }

# Async runtime
tokio = { version = "1", features = ["full"] }

# HTTP client for AI API
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Terminal UI
crossterm = "0.27"
dialoguer = "0.11"
console = "0.15"
colored = "2"

# Hotkey support
rdev = "0.5"
# Alternative: global-hotkey = "0.5"

# Configuration
config = "0.13"
toml = "0.8"
dirs = "5"

# Shell execution
shell-words = "1"

# Error handling
anyhow = "1"
thiserror = "1"

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"
```

### B. AI Client Architecture

```rust
// ai/client.rs
pub struct ClaudeClient {
    api_key: String,
    model: String,
    base_url: String,
}

impl ClaudeClient {
    pub async fn generate_command(&self, prompt: &str, context: &Context) -> Result<String> {
        // System prompt: "Reply ONLY with executable shell commands"
        // Include context: current dir, git branch, shell type
    }

    pub async fn answer_question(&self, prompt: &str) -> Result<String> {
        // System prompt: "Provide helpful, concise answers"
    }

    pub async fn explain_command(&self, cmd: &str) -> Result<String> {
        // Explain each part of the command
    }
}

// ai/parser.rs
pub fn extract_commands(response: &str) -> Vec<String> {
    // Parse code blocks (```bash, ```sh, etc.)
    // Extract commands line by line
    // Handle multi-line commands (backslash continuation)
    // Preserve order and dependencies
}

pub fn parse_command_sequence(commands: Vec<String>) -> Vec<CommandStep> {
    // Detect dependencies (e.g., cd before ls)
    // Identify parallel vs sequential execution
}
```

### C. Command Execution & Safety

```rust
// executor/validator.rs
pub struct SafetyValidator {
    dangerous_patterns: Vec<Regex>,
    requires_confirmation: Vec<Regex>,
}

impl SafetyValidator {
    pub fn validate(&self, cmd: &str) -> ValidationResult {
        // Check for dangerous commands
        // - rm -rf /
        // - dd if=/dev/zero
        // - mkfs.*
        // - curl ... | bash
        // - sudo without context

        ValidationResult {
            is_safe: bool,
            risk_level: RiskLevel,  // Low, Medium, High, Critical
            warning: Option<String>,
            requires_confirmation: bool,
        }
    }
}

// executor/shell.rs
pub struct ShellExecutor {
    shell: String,  // bash, zsh, fish
    dry_run: bool,
}

impl ShellExecutor {
    pub async fn execute(&self, cmd: &str) -> Result<ExecutionResult> {
        // Execute command
        // Capture stdout, stderr
        // Return exit code
    }

    pub async fn execute_sequence(&self, cmds: Vec<String>) -> Result<Vec<ExecutionResult>> {
        // Execute commands in order
        // Stop on first failure (unless force continue)
    }
}
```

### D. Global Hotkey Listener (macOS)

```rust
// ui/hotkey.rs
pub struct HotkeyDaemon {
    hotkeys: HashMap<Hotkey, HotkeyAction>,
}

impl HotkeyDaemon {
    pub fn new() -> Self {
        // Register global hotkeys
        // Cmd+J → DO mode
        // Cmd+Shift+J → ASK mode
    }

    pub async fn run(&self) -> Result<()> {
        // Listen for hotkey events
        // On trigger, spawn UI/input interface
        // Run in background as daemon
    }
}

// Integration with launchd (macOS)
// ~/.config/aido/com.aido.daemon.plist
```

### E. UI Options

#### Option 1: Terminal-Based (MVP)

```rust
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

pub fn prompt_user(prompt: &str) -> Result<String> {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact_text()
}

pub fn confirm_execution(cmd: &str, warning: Option<String>) -> Result<ConfirmAction> {
    // Show command preview
    // Display warning if dangerous
    // Offer options: Execute, Cancel, Edit, Explain
}
```

#### Option 2: Native Popup (Future)

- Use `tauri` for cross-platform native UI
- Floating window like Spotlight/Alfred
- Custom styling and animations

---

## Configuration

### Config File: `~/.config/aido/config.toml`

```toml
[ai]
api_key = "sk-ant-..."
model = "claude-sonnet-4"
provider = "anthropic"
max_tokens = 1024
temperature = 0.7

[hotkeys]
do_mode = "cmd+j"
ask_mode = "cmd+shift+j"
enabled = true

[execution]
always_confirm = true
auto_explain = false
dangerous_commands = ["rm", "mv", "dd", "mkfs", "format"]
dry_run = false

[ui]
style = "terminal"  # or "popup"
color_scheme = "auto"  # auto, dark, light
show_context = true

[logging]
level = "info"
file = "~/.config/aido/aido.log"
```

### Environment Variables

```bash
export AIDO_API_KEY="sk-ant-..."
export AIDO_MODEL="claude-sonnet-4"
export AIDO_CONFIG="~/.config/aido/config.toml"
```

---

## CLI Interface

### Installation

```bash
# Install via Homebrew
brew tap yourusername/aido
brew install aido

# Or build from source
git clone https://github.com/yourusername/aido
cd aido
cargo build --release
cargo install --path .
```

### Setup

```bash
# Initialize configuration
aido init

# Configure API key
aido config set ai.api_key sk-ant-...

# Or use interactive setup
aido setup
```

### Usage

```bash
# Direct command generation (DO mode)
aido do "find all Python files modified today"
aido do "create a new React component"

# Ask questions (ASK mode)
aido ask "what is the difference between curl and wget?"
aido ask "how to use rsync?"

# Start/stop daemon for hotkey support
aido daemon start
aido daemon stop
aido daemon status
aido daemon restart

# Configuration management
aido config show
aido config edit
aido config set execution.always_confirm false

# History
aido history                # Show recent commands
aido history --limit 20     # Show last 20
aido rerun 3                # Re-run command #3 from history

# Utilities
aido --version
aido --help
aido doctor                 # Check configuration and dependencies
```

---

## Command Extraction & Parsing

### Challenge: Extract Commands from AI Responses

AI might return commands in various formats:

```
Option 1 (code block):
```bash
mkdir my-project
cd my-project
git init
```

Option 2 (inline):
You can do this with: `mkdir my-project && cd my-project && git init`

Option 3 (mixed):
First, create the directory:
mkdir my-project

Then initialize git:
cd my-project
git init
```

### Parsing Strategy

```rust
pub struct CommandExtractor {
    patterns: Vec<ExtractionPattern>,
}

impl CommandExtractor {
    pub fn extract(&self, text: &str) -> Vec<String> {
        // 1. Look for code blocks (```bash, ```sh, ```)
        // 2. Look for inline commands (`command`)
        // 3. Look for command-like lines (start with common commands)
        // 4. Filter out explanations and non-commands
        // 5. Preserve order
    }

    pub fn detect_sequence_type(&self, cmds: &[String]) -> SequenceType {
        // Sequential: Commands depend on each other (cd, then ls)
        // Parallel: Independent commands
        // Conditional: If-then logic
    }
}
```

### Multi-Command Handling

```rust
pub enum CommandSequence {
    Single(String),
    Sequential(Vec<String>),      // Execute in order, stop on failure
    Parallel(Vec<String>),         // Execute concurrently
    Conditional(Vec<ConditionalCmd>),  // If-then-else logic
}

pub struct ConditionalCmd {
    condition: String,
    on_success: String,
    on_failure: Option<String>,
}
```

---

## Safety & Security

### Dangerous Command Detection

```rust
const DANGEROUS_PATTERNS: &[&str] = &[
    r"rm\s+(-rf?|--recursive|--force).*(/|\*)",  // rm -rf /
    r"dd\s+.*of=/dev/[hs]d[a-z]",                // dd to disk
    r"mkfs\.",                                    // Format filesystem
    r":\(\)\{.*;\};:",                           // Fork bomb
    r"curl.*\|\s*(bash|sh)",                     // Pipe to shell
    r"wget.*-O-.*\|\s*(bash|sh)",                // Pipe to shell
    r">\s*/dev/sd[a-z]",                         // Write to disk
];

const REQUIRES_CONFIRMATION: &[&str] = &[
    r"^rm\s+",
    r"^mv\s+",
    r"^sudo\s+",
    r"git\s+push.*--force",
    r"git\s+reset.*--hard",
    r"^chmod\s+",
    r"^chown\s+",
];
```

### Confirmation UI

```
┌──────────────────────────────────────────────┐
│ ⚠️  DANGEROUS COMMAND DETECTED                │
│                                              │
│ ▶ rm -rf old_project/                       │
│                                              │
│ This command will:                           │
│ • DELETE files recursively                   │
│ • Cannot be undone                           │
│                                              │
│ Type 'DELETE' to confirm:                    │
│ [________]                                   │
│                                              │
│ Or: [e] Edit  [c] Cancel  [x] Explain        │
└──────────────────────────────────────────────┘
```

### Audit Logging

All commands logged to `~/.config/aido/history.log`:

```
2025-11-05 14:23:45 | DO | find . -type f -size +100M | SUCCESS | exit_code=0
2025-11-05 14:24:12 | DO | rm old_file.txt | SUCCESS | exit_code=0
2025-11-05 14:25:03 | ASK | what does rsync do? | N/A | N/A
```

---

## Context Awareness

Enhance AI responses by providing context:

```rust
pub struct ExecutionContext {
    pub current_dir: PathBuf,
    pub shell: String,              // bash, zsh, fish
    pub git_branch: Option<String>,
    pub git_repo: Option<String>,
    pub recent_files: Vec<String>,
    pub env_vars: HashMap<String, String>,
    pub os: String,
}

impl ExecutionContext {
    pub fn collect() -> Result<Self> {
        // Gather context information
    }

    pub fn to_prompt(&self) -> String {
        format!(
            "Current directory: {}\nShell: {}\nGit branch: {}\nOS: {}",
            self.current_dir.display(),
            self.shell,
            self.git_branch.as_deref().unwrap_or("none"),
            self.os
        )
    }
}
```

---

## Implementation Phases

### Phase 1: MVP (Week 1-2)

**Goal**: Basic working CLI tool

- [x] Project setup with Cargo
- [ ] CLI argument parsing (clap)
- [ ] Basic DO mode: `aido do "command"`
- [ ] Basic ASK mode: `aido ask "question"`
- [ ] Claude API integration
- [ ] Simple command extraction
- [ ] Terminal-based confirmation prompt
- [ ] Basic safety validation
- [ ] Configuration file support

**Deliverable**: Working CLI tool that can be invoked manually

### Phase 2: Hotkey Support (Week 3)

**Goal**: Global hotkey triggering

- [ ] Global hotkey listener (rdev)
- [ ] Background daemon process
- [ ] macOS launchd integration
- [ ] Daemon start/stop/status commands
- [ ] Input capture from anywhere
- [ ] Terminal window spawning

**Deliverable**: Press Cmd+J to trigger from anywhere

### Phase 3: Polish & Safety (Week 4)

**Goal**: Production-ready with safety features

- [ ] Advanced safety validators
- [ ] Dangerous command warnings
- [ ] Command history tracking
- [ ] Edit-before-execute feature
- [ ] Explain command feature
- [ ] Multi-command sequence handling
- [ ] Better error handling and messages
- [ ] Configuration management commands
- [ ] Comprehensive testing

**Deliverable**: Safe, polished tool ready for daily use

### Phase 4: Advanced Features (Future)

**Goal**: Enhanced UX and capabilities

- [ ] Native popup UI (Tauri)
- [ ] Context awareness (git status, recent files)
- [ ] Learning from user corrections
- [ ] Shell integration (zsh/bash plugin)
- [ ] Templating system for common tasks
- [ ] Multiple AI provider support (OpenAI, Ollama)
- [ ] Team/shared configurations
- [ ] Analytics dashboard

---

## Homebrew Formula

### aido.rb

```ruby
class Aido < Formula
  desc "AI-powered command generation and execution for your terminal"
  homepage "https://github.com/yourusername/aido"
  url "https://github.com/yourusername/aido/archive/refs/tags/v1.0.0.tar.gz"
  sha256 "YOUR_SHA256_HERE"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args

    # Install shell completions
    generate_completions_from_executable(bin/"aido", "completions")

    # Install man page
    man1.install "docs/aido.1"
  end

  def post_install
    # Create config directory
    (var/"aido").mkpath

    # Setup launchd daemon (optional)
    if OS.mac?
      puts "To enable hotkey support, run:"
      puts "  aido daemon start"
    end
  end

  def caveats
    <<~EOS
      To get started:
        1. Run 'aido init' to configure your API key
        2. Run 'aido daemon start' to enable hotkey support
        3. Press Cmd+J to trigger DO mode
        4. Press Cmd+Shift+J to trigger ASK mode

      For help: aido --help
    EOS
  end

  test do
    assert_match "aido", shell_output("#{bin}/aido --version")
    system "#{bin}/aido", "doctor"
  end
end
```

### Installation & Distribution

```bash
# Create your tap
brew tap yourusername/aido https://github.com/yourusername/homebrew-aido

# Install
brew install yourusername/aido/aido

# Update
brew upgrade aido

# Uninstall
brew uninstall aido
```

---

## Advanced Features (Future)

### 1. Learning from Corrections

```rust
// When user edits a command, learn from it
pub struct LearningEngine {
    corrections: Vec<Correction>,
}

pub struct Correction {
    original_prompt: String,
    ai_generated: String,
    user_edited: String,
    timestamp: DateTime<Utc>,
}

// Include corrections in future prompts
// "Previous similar request: ... You generated: ... User corrected to: ..."
```

### 2. Template System

```bash
# Save common operations as templates
aido template save "new-react-component" \
  "mkdir components/{name} && touch components/{name}/{name}.tsx"

# Use templates
aido template run new-react-component --name Button
```

### 3. Pipeline Mode

```bash
# Chain commands with AI assistance
ls | aido do "filter only PDF files" | aido do "sort by size"
```

### 4. Interactive Mode

```bash
aido interactive
> find large files
▶ find . -type f -size +100M
[Execute]
> now delete them
▶ find . -type f -size +100M -delete
⚠️  This will delete files! Confirm? [y/N]
```

---

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_command_extraction() {
        let response = "```bash\nls -la\npwd\n```";
        let cmds = extract_commands(response);
        assert_eq!(cmds, vec!["ls -la", "pwd"]);
    }

    #[test]
    fn test_dangerous_command_detection() {
        let validator = SafetyValidator::new();
        assert!(validator.is_dangerous("rm -rf /"));
        assert!(!validator.is_dangerous("ls -la"));
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_full_workflow() {
    // Mock AI client
    // Test: prompt → extraction → validation → execution
}
```

### Manual Testing Checklist

- [ ] DO mode generates correct commands
- [ ] ASK mode returns helpful answers
- [ ] Dangerous commands show warnings
- [ ] Multi-command sequences execute in order
- [ ] Hotkeys work from different applications
- [ ] Configuration persists across restarts
- [ ] History tracking works correctly

---

## Success Metrics

- **Accuracy**: 90%+ of generated commands work without modification
- **Safety**: 100% of dangerous commands require confirmation
- **Speed**: <2s from hotkey press to command display
- **Reliability**: 99%+ uptime for daemon

---

## Contributing & Next Steps

### Getting Started

1. Clone the repository
2. Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
3. Build: `cargo build`
4. Run: `cargo run -- do "list files"`

### Development Workflow

```bash
# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run -- do "test command"

# Format code
cargo fmt

# Lint
cargo clippy
```

---

## Resources

- **Rust Book**: https://doc.rust-lang.org/book/
- **Clap (CLI)**: https://docs.rs/clap/
- **Tokio (Async)**: https://tokio.rs/
- **Dialoguer (UI)**: https://docs.rs/dialoguer/
- **Claude API**: https://docs.anthropic.com/

---

## License

MIT License - See LICENSE file for details

---

**Let's build this!** 🚀
