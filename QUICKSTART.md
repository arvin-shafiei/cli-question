# AIDO Quick Start Guide

## 🚀 Installation (2 minutes)

### Step 1: Run the installer
```bash
cd /Users/arvin/Documents/cli-question
./install.sh
```

### Step 2: Reload your shell
```bash
source ~/.zshrc
# or just restart your terminal
```

### Step 3: Test it works
```bash
aido --version
aido doctor
```

That's it! AIDO is now installed and ready to use.

---

## 🎯 Basic Usage

### Method 1: Command Line

**Generate and run commands:**
```bash
aido do "list all files in current directory"
```

**Ask questions:**
```bash
aido ask "what does the ls command do?"
```

### Method 2: Keybindings (Recommended)

**Setup once:**
Add this to your `~/.zshrc` file:
```bash
eval "$(aido setup-shell)"
```

Then reload: `source ~/.zshrc`

**Use it:**
- Press `Ctrl+O` anywhere in your terminal for DO mode
- Or press `Ctrl+P` for ASK mode
- Type what you want to do (e.g., "show disk usage")
- Press Enter
- Review and confirm the command
- Done!

---

## 📝 Common Examples

```bash
# Find files
aido do "find all PDF files larger than 10MB"

# Git operations
aido do "show git status and recent commits"

# System info
aido do "show memory usage"

# Questions
aido ask "how to use rsync safely?"
```

---

## 🔑 Key Features

- ✅ Uses `claude -p` - no API key setup needed!
- ✅ Safety checks - warns before dangerous commands
- ✅ Always confirms before executing
- ✅ Works with any shell (bash, zsh, fish)
- ✅ Keybinding support for quick access

---

## 🎨 Command Options

```bash
aido do "command" -y    # Auto-execute without confirmation
aido do "command" -n    # Dry run (show but don't execute)
aido do "command" -v    # Verbose mode
```

---

## 🛠️ Troubleshooting

**"aido: command not found"**
```bash
source ~/.zshrc  # Reload your shell config
```

**"Claude CLI not found"**
```bash
# Make sure Claude Code CLI is installed
claude --version
```

**Check everything:**
```bash
aido doctor
```

---

## 📚 Next Steps

1. ✅ Install AIDO (you did this!)
2. ⬜ Set up keybindings (`eval "$(aido setup-shell)"`)
3. ⬜ Try a few DO commands
4. ⬜ Try ASK mode for questions
5. ⬜ Read the full README for advanced features

---

**You're all set! Start using AIDO now:**

```bash
aido do "show me the largest files in my Downloads folder"
```

Happy coding! 🎉
